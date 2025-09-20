//! Terminal User interface (TUI) module
//!
//! From the Canvas example in Ratatui's repository:
//! https://github.com/ratatui/ratatui/blob/main/examples/apps/canvas/src/main.rs
//!

use crate::{
    ACTIVE_UNITS_GROUP_IDX, SC2EventIteratorItem,
    state::{SC2EventIterator, SC2EventType, UnitChangeHint},
    tracker_events::unit_tag_index,
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crate::syntect_json_highlight;
use color_eyre::Result;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, MouseEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{self, Line, Span},
    widgets::{
        Block, Paragraph, Widget, Wrap,
        canvas::{Canvas, Circle, Rectangle},
    },
};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect_tui::translate_style;

pub fn ratatui_main(
    sc2_event_iter: SC2EventIterator,
    details: crate::details::Details,
    init_data: crate::init_data::InitData,
    syntect_syntax_set: SyntaxSet,
    syntect_theme_set: ThemeSet,
) -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let terminal = ratatui::init();
    let app_result = S2ProtoRatatuiApp::new(
        sc2_event_iter,
        details,
        init_data,
        syntect_syntax_set,
        syntect_theme_set,
    )
    .run(terminal);
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
    tick_count: u64,
    marker: Marker,
    points: Vec<Position>,
    is_drawing: bool,
    details: crate::details::Details,
    init_data: crate::init_data::InitData,
    sc2_event_iter: SC2EventIterator,
    current_event: Option<SC2EventIteratorItem>,
    syntect_syntax_set: SyntaxSet,
    syntect_theme_set: ThemeSet,
}

impl S2ProtoRatatuiApp {
    fn new(
        mut sc2_event_iter: SC2EventIterator,
        details: crate::details::Details,
        init_data: crate::init_data::InitData,
        syntect_syntax_set: SyntaxSet,
        syntect_theme_set: ThemeSet,
    ) -> Self {
        let mut current_event = sc2_event_iter.next();
        loop {
            let mut should_break = true;
            match &current_event {
                Some(event) => match event.event_type {
                    SC2EventType::Tracker { tracker_loop, .. } => {
                        if tracker_loop == 0 {
                            should_break = false;
                        }
                    }
                    SC2EventType::Game { game_loop, .. } => {
                        if game_loop == 0 {
                            should_break = false;
                        }
                    }
                },
                None => break,
            }
            if should_break {
                break;
            }
            current_event = sc2_event_iter.next();
        }
        Self {
            exit: false,
            x: 0.0,
            y: 0.0,
            tick_count: 0,
            marker: Marker::Dot,
            points: vec![],
            is_drawing: false,
            details,
            init_data,
            sc2_event_iter,
            current_event,
            syntect_syntax_set,
            syntect_theme_set,
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
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
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
            // self.current_event = self.sc2_event_iter.next();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let vertical = Layout::vertical([Constraint::Percentage(25), Constraint::Percentage(75)]);
        let [left, right] = horizontal.areas(frame.area());
        let [draw, map] = vertical.areas(left);
        let [details, player_selection_area] = vertical.areas(right);
        let [player1_selected_units, player2_selected_units] =
            vertical.areas(player_selection_area);

        frame.render_widget(self.map_canvas(), map);
        frame.render_widget(self.event_canvas(), draw);
        frame.render_widget(self.details_canvas(), details);
        frame.render_widget(self.selected_player_units(0), player1_selected_units);
        frame.render_widget(self.selected_player_units(1), player2_selected_units);
    }

    fn event_canvas(&self) -> impl Widget + '_ {
        let mut text: Vec<Line<'_>> = vec![];
        let mut spans = vec![];
        if let Some(event_item) = &self.current_event {
            match event_item.event_type {
                SC2EventType::Tracker {
                    ref event,
                    tracker_loop,
                } => {
                    let game_secs = tracker_loop as f64 / 22.0;
                    text.push(text::Line::from(Span::from(format!(
                        "TRCK[{game_secs:>10.3}s][{tracker_loop:0>8}]: "
                    ))));
                    for (style, data) in syntect_json_highlight(
                        serde_json::to_string(&event).unwrap().as_str(),
                        &self.syntect_syntax_set,
                        &self.syntect_theme_set,
                    ) {
                        spans.push(Span::styled(
                            String::from(data),
                            translate_style(style).unwrap(),
                        ))
                    }
                    text.push(Line::from(spans));
                }
                SC2EventType::Game {
                    ref event,
                    user_id,
                    game_loop,
                } => {
                    text.push(text::Line::from(Span::styled(
                        format!(
                            "UID[{user_id:<02}{:>20}]",
                            self.details.get_player_name(user_id as u8)
                        ),
                        Style::default().fg(self.get_player_color(user_id)),
                    )));
                    let game_secs = game_loop as f64 / 22.0;
                    text.push(text::Line::from(Span::from(format!(
                        "GAME[{game_secs:>10.3}s][{game_loop:0>8}]: "
                    ))));
                    for (style, data) in syntect_json_highlight(
                        serde_json::to_string(&event).unwrap().as_str(),
                        &self.syntect_syntax_set,
                        &self.syntect_theme_set,
                    ) {
                        spans.push(Span::styled(
                            String::from(data),
                            translate_style(style).unwrap(),
                        ))
                    }
                    text.push(Line::from(spans));
                }
            }
            match &event_item.change_hint {
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
                        "[{unit:>15}]. creator: {creator}",
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
                UnitChangeHint::Abilities {
                    units,
                    event,
                    target: _,
                } => {
                    text.push(text::Line::from(Span::styled(
                        "Abilities",
                        Style::default().fg(Color::Red),
                    )));
                    for unit in units {
                        // Format the unit abilities for display
                        let unit = format!("{:>8}:{:>8}", unit.tag_index, unit.name);
                        text.push(text::Line::from(Span::from(format!(
                            "Unit: {unit} (command: {event:?})"
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
        if let Some(current_event) = &self.current_event {
            let mut spans = vec![];
            for (style, data) in syntect_json_highlight(
                serde_json::to_string(&current_event.change_hint)
                    .unwrap()
                    .as_str(),
                &self.syntect_syntax_set,
                &self.syntect_theme_set,
            ) {
                spans.push(Span::styled(
                    String::from(data),
                    translate_style(style).unwrap(),
                ))
            }
            text.push(Line::from(spans));
        }
        let block = Block::bordered().title(Span::styled(
            "Event",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        Paragraph::new(text).block(block).wrap(Wrap { trim: true })
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        let map_size_x = self.init_data.sync_lobby_state.game_description.map_size_x;
        let map_size_y = self.init_data.sync_lobby_state.game_description.map_size_y;
        Canvas::default()
            .block(Block::bordered().title(format!(
                "{} - size {}x{}",
                self.sc2_event_iter.sc2_state.filename, map_size_x, map_size_y
            )))
            .marker(self.marker)
            .x_bounds([0.0, f64::from(map_size_x)])
            .y_bounds([0.0, f64::from(map_size_y)])
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
                        color: self.get_player_color(*user_id),
                    });
                }
            })
    }

    fn selected_player_units(&self, user_index: usize) -> impl Widget + '_ {
        let mut text: Vec<Line<'_>> = vec![];
        for (idx, (_uid, user_state)) in self.sc2_event_iter.sc2_state.user_state.iter().enumerate()
        {
            if user_index != idx {
                continue;
            }
            for selected_unit in user_state.control_groups[ACTIVE_UNITS_GROUP_IDX].clone() {
                let unit_index = unit_tag_index(selected_unit as i64);
                if let Some(registered_unit) = self.sc2_event_iter.sc2_state.units.get(&unit_index)
                {
                    text.push(text::Line::from(format!(
                        "{}[@{}]:{}",
                        registered_unit.name,
                        registered_unit.tag_index,
                        registered_unit
                            .cmd
                            .abil
                            .clone()
                            .map(|a| a.ability)
                            .unwrap_or("".to_string())
                    )));
                }
            }
        }
        let block = Block::bordered().title(Span::styled(
            format!("{user_index} Selected Units"),
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ));
        Paragraph::new(text).block(block).wrap(Wrap { trim: true })
    }

    fn get_player_color(&self, user_id: i64) -> Color {
        self.details
            .player_list
            .iter()
            .find(|player| player.working_set_slot_id == Some(user_id as u8))
            .map_or(Color::White, |player| {
                Color::Rgb(player.color.r, player.color.g, player.color.b)
            })
    }

    fn details_canvas(&self) -> impl Widget + '_ {
        let mut text: Vec<Line<'_>> = vec![];
        for player in &self.details.player_list {
            if let Some(slot_id) = player.working_set_slot_id {
                text.push(text::Line::from(Span::styled(
                    format!(
                        "{}/{}/{}",
                        player.toon.region, player.toon.realm, player.toon.id
                    ),
                    Style::default().fg(self.get_player_color(i64::from(slot_id))),
                )));
                text.push(text::Line::from(Span::styled(
                    format!(" - {:>32} Slot: {}", player.name, slot_id),
                    Style::default().fg(self.get_player_color(i64::from(slot_id))),
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
