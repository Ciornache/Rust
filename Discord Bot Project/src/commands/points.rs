use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use rusqlite::Connection;

pub fn run(_options: &[ResolvedOption]) -> Result<String, rusqlite::Error> {
    let conn= Connection::open("users.db")?;
    let mut leaderboard = String::new();
    let select_statement = "SELECT name, points FROM users ORDER BY points DESC";
    let mut stmt = conn.prepare(select_statement)?;
    let mut it = stmt.query_map([], |row| -> Result<(String, i32), rusqlite::Error>  {
        let name:String  = row.get(0)?;
        let points: i32 = row.get(1)?;
        return Ok((name, points));
    })?;

    leaderboard.push_str("Leaderboard\n");
    let mut rank_id = 0;
    loop {
        rank_id = rank_id + 1;
        let row = it.next();
        if row == None {
            break;
        }
        if let Some(element) = row {
            if let Ok(res) = element 
            {
                let name = res.0;
                let points = res.1;
                leaderboard.push_str(format!("{}. {} - {}\n", rank_id, name.as_str(), points).as_str());
            }
        }
    }

    return Ok(leaderboard);
}

pub fn register() -> CreateCommand {
    CreateCommand::new("points")
        .description("Prints the leaderboard")
}
