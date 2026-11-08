//! The example of the README: the map and the players of a replay.
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(std::env::args().nth(1).expect("a replay"));
    let contents = s2protocol::read_file(&path)?;
    let (_, mpq) = s2protocol::parser::parse(&contents).map_err(|e| format!("{e:?}"))?;
    let details = s2protocol::read_details(&path.to_string_lossy(), &mpq, &contents)?;
    println!("{}", details.title);
    for player in &details.player_list {
        println!("{} ({})", player.name, player.race);
    }
    Ok(())
}
