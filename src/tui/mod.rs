//! Terminal User interface (TUI) module
//!
//! From the Canvas example in Ratatui's repository:
//! https://github.com/ratatui/ratatui/blob/main/examples/apps/canvas/src/main.rs
//!

use crate::{
    game_events::ReplayGameEvent,
    state::{SC2EventIterator, SC2EventType, UnitChangeHint},
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},
    ExecutableCommand,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, MouseEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{self, Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Points, Rectangle},
        Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
        Row, Sparkline, Table, Tabs, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};

pub fn ratatui_main(
    sc2_event_iter: SC2EventIterator,
    details: crate::details::Details,
) -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = ratatui::init();
    let app_result = S2ProtoRatatuiApp::new(sc2_event_iter, details).run(terminal);
    ratatui::restore();
    stdout().execute(DisableMouseCapture)?;
    app_result
}

use std::fmt;

pub const GAME_SPOINT_MINI_TO_MAP_RATIO: f64 = 250.;

pub struct PrettyPrintBuffer(pub Vec<String>);

impl fmt::Write for PrettyPrintBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.push(s.to_string());
        Ok(())
    }
}

struct S2ProtoRatatuiApp {
    exit: bool,
    x: f64,
    y: f64,
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    marker: Marker,
    points: Vec<Position>,
    is_drawing: bool,
    details: crate::details::Details,
    sc2_event_iter: SC2EventIterator,
    current_event: Option<(SC2EventType, UnitChangeHint)>,
}

impl S2ProtoRatatuiApp {
    const fn new(sc2_event_iter: SC2EventIterator, details: crate::details::Details) -> Self {
        Self {
            exit: false,
            x: 0.0,
            y: 0.0,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(10, 10, 200, 100),
            vx: 1.0,
            vy: 1.0,
            tick_count: 0,
            marker: Marker::Dot,
            points: vec![],
            is_drawing: false,
            details,
            sc2_event_iter,
            current_event: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_press(key),
                    Event::Mouse(event) => self.handle_mouse_event(event),
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Down | KeyCode::Char('j') => self.y += 1.0,
            KeyCode::Char('n') | KeyCode::Right | KeyCode::Char(' ') => {
                self.current_event = self.sc2_event_iter.next()
            }
            KeyCode::Up | KeyCode::Char('k') => self.y -= 1.0,
            KeyCode::Char('l') => self.x += 1.0,
            KeyCode::Left | KeyCode::Char('h') => self.x -= 1.0,
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, event: event::MouseEvent) {
        match event.kind {
            MouseEventKind::Down(_) => self.is_drawing = true,
            MouseEventKind::Up(_) => self.is_drawing = false,
            MouseEventKind::Drag(_) => {
                self.points.push(Position::new(event.column, event.row));
            }
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        if (self.tick_count % 10) == 0 {
            /*self.current_event = self.sc2_event_iter.next();
            *self.marker = match self.marker {
                Marker::Dot => Marker::Braille,
                Marker::Braille => Marker::Block,
                Marker::Block => Marker::HalfBlock,
                Marker::HalfBlock => Marker::Bar,
                Marker::Bar => Marker::Dot,
            };*/
        }
        // bounce the ball by flipping the velocity vector
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let vertical = Layout::vertical([Constraint::Percentage(25), Constraint::Percentage(75)]);
        let [left, right] = horizontal.areas(frame.area());
        let [draw, map] = vertical.areas(left);
        let [pong, _right_bottom_pane] = vertical.areas(right);

        frame.render_widget(self.map_canvas(), map);
        frame.render_widget(self.event_canvas(), draw);
        frame.render_widget(self.replay_canvas(), pong);
    }

    fn event_canvas(&self) -> impl Widget + '_ {
        let mut text: Vec<Line<'_>> = vec![];
        if let Some((event_type, hint)) = &self.current_event {
            match event_type {
                SC2EventType::Tracker {
                    event: _,
                    tracker_loop,
                } => {
                    let game_secs = *tracker_loop as f64 / 22.0;
                    text.push(text::Line::from(Span::from(format!(
                        "TRCK[{game_secs:>10.3}s][{tracker_loop:0>8}]: "
                    ))));
                }
                SC2EventType::Game {
                    event: _,
                    user_id: _,
                    game_loop,
                } => {
                    let game_secs = *game_loop as f64 / 22.0;
                    text.push(text::Line::from(Span::from(format!(
                        "GAME[{game_secs:>10.3}s][{game_loop:0>8}]: "
                    ))));
                }
            }
            match hint {
                UnitChangeHint::Registered { unit, creator } => {
                    text.push(text::Line::from(Span::styled(
                        "Registered: ",
                        Style::default().fg(Color::LightGreen),
                    )));
                    let ability = unit
                        .creator_ability_name
                        .as_ref()
                        .map(|name| format!("Unit ability: {name}"))
                        .unwrap_or_else(|| "".to_string());
                    let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                    let creator = creator
                        .as_ref()
                        .map(|creator| {
                            format!("{:>8}:{:>8}{ability}", creator.tag_index, creator.name)
                        })
                        .unwrap_or(format!("None{ability}"));

                    text.push(text::Line::from(Span::from(format!(
                        "Unit: {unit}. creator: {creator}"
                    ))));
                }
                UnitChangeHint::Positions(units) => {
                    text.push(text::Line::from(Span::styled(
                        "Unit positions: ",
                        Style::default().fg(Color::Cyan),
                    )));
                    for unit in units {
                        // Format the unit position for display
                        let pos = &unit.pos;
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!(
                            "Unit: {unit} (pos: {pos:?})"
                        ))));
                    }
                }
                UnitChangeHint::TargetPoints(units) => {
                    text.push(text::Line::from(Span::styled(
                        "Target Points: ",
                        Style::default().fg(Color::LightRed),
                    )));
                    for unit in units {
                        // Format the unit target for display
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!("Unit: {unit}"))));
                    }
                }
                UnitChangeHint::TargetUnits { units, target } => {
                    text.push(text::Line::from(Span::styled(
                        "Target Units",
                        Style::default().fg(Color::Yellow),
                    )));
                    let target = format!("{:>8}:{:>8}", target.tag_index, target.name);
                    for unit in units {
                        // Format the unit target for display
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!(
                            "Unit: {unit} (target: {target})"
                        ))));
                    }
                }
                UnitChangeHint::Unregistered { killer, killed } => {
                    text.push(text::Line::from(Span::styled(
                        "Unregister",
                        Style::default().fg(Color::Red),
                    )));
                    let killed = format!("{:>8}:{:>8}", killed.tag_index, killed.name);
                    let killer = killer
                        .as_ref()
                        .map(|killer| format!("Killer: {:>8}:{:>8}", killer.tag_index, killer.name))
                        .unwrap_or_else(|| "".to_string());

                    text.push(text::Line::from(Span::from(format!(
                        "Unit: {killed:0>8} ({killer:0>8})"
                    ))));
                }
                UnitChangeHint::Abilities(units, cmd) => {
                    text.push(text::Line::from(Span::styled(
                        "Abilities",
                        Style::default().fg(Color::Red),
                    )));
                    for unit in units {
                        // Format the unit abilities for display
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!(
                            "Unit: {unit} (command: {cmd:?})"
                        ))));
                    }
                }
                UnitChangeHint::Selection(units) => {
                    text.push(text::Line::from(Span::styled(
                        "Selection",
                        Style::default().fg(Color::White),
                    )));
                    for unit in units {
                        // Format the unit selection for display
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!("Unit: {unit}"))));
                    }
                }
                UnitChangeHint::None => {
                    text.push(text::Line::from(Span::from("No unit change hint")));
                }
            }
        }
        text.push(text::Line::from(""));
        text.push(text::Line::from(
            serde_json::to_string(&self.current_event).unwrap(),
        ));

        let block = Block::bordered().title(Span::styled(
            "Event",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        Paragraph::new(text).block(block).wrap(Wrap { trim: true })
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title(self.sc2_event_iter.sc2_state.filename.clone()))
            .marker(self.marker)
            .x_bounds([50.0, 200.0])
            .y_bounds([50.0, 200.0])
            .paint(move |ctx| {
                for unit in self.sc2_event_iter.sc2_state.units.values() {
                    ctx.draw(&Circle {
                        x: f64::from(unit.pos.x()),
                        y: f64::from(unit.pos.y()),
                        radius: f64::from(unit.radius),
                        color: Color::Rgb(unit.color[0], unit.color[1], unit.color[2]),
                    });
                }
                for (user_id, user) in &self.sc2_event_iter.sc2_state.user_state {
                    ctx.draw(&Rectangle {
                        x: (user.camera_pos.x as f64 / GAME_SPOINT_MINI_TO_MAP_RATIO) - 10.0,
                        y: (user.camera_pos.y as f64 / GAME_SPOINT_MINI_TO_MAP_RATIO) - 10.0,
                        width: 20.0,
                        height: 20.0,
                        color: self.get_player_color(user_id),
                    });
                }
            })
    }

    fn get_player_color(&self, user_id: &i64) -> Color {
        self.details
            .player_list
            .iter()
            .find(|player| player.working_set_slot_id == Some(*user_id as u8))
            .map_or(Color::White, |player| {
                Color::Rgb(player.color.r, player.color.g, player.color.b)
            })
    }

    fn replay_canvas(&self) -> impl Widget + '_ {
        let mut text: Vec<Line<'_>> = vec![];
        for player in &self.details.player_list {
            if let Some(slot_id) = player.working_set_slot_id {
                text.push(text::Line::from(Span::styled(
                    format!(
                        "{}/{}/{}",
                        player.toon.region, player.toon.realm, player.toon.id
                    ),
                    Style::default().fg(self.get_player_color(&i64::from(slot_id))),
                )));
                text.push(text::Line::from(Span::styled(
                    format!(" - {:>32} Slot: {}", player.name, slot_id),
                    Style::default().fg(self.get_player_color(&i64::from(slot_id))),
                )));
                text.push(text::Line::from(Span::styled(
                    format!("- {:>24} Result: {}", player.race, player.result),
                    Style::default().fg(Color::White),
                )));
            }
        }

        let block = Block::bordered().title(Span::styled(
            "Replay",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        Paragraph::new(text).block(block).wrap(Wrap { trim: true })
    }
}
