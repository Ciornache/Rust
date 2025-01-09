use rusqlite::{params, Connection, Result};
pub async fn init_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let create_statement = r"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name TEXT NOT NULL UNIQUE, 
            points INTEGER DEFAULT 0
        );
    ";
    conn.execute(create_statement, [])?;
    Ok(())
}

pub async fn insert_user_into_database(user_name: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let select_statement = "SELECT EXISTS(SELECT 1 FROM users WHERE name = ?1);";
    let insert_statement = "INSERT INTO users (name, points) VALUES (?1, ?2);";
    let mut stmt = conn.prepare(select_statement)?;
    let exists: bool = stmt.query_row(params![user_name], |row| row.get(0))?;
    if !exists {
        conn.execute(insert_statement, params![user_name, 0])?;
    }
    Ok(())
}

pub async fn update_user(user_name: String, number_of_points: i32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let update_statement = r"
        UPDATE users
        SET points = points + ?1
        WHERE name = ?2;
    ";
    let rows_affected = conn.execute(update_statement, params![number_of_points, user_name])?;
    if rows_affected == 0 {
        Err(rusqlite::Error::QueryReturnedNoRows)
    } else {
        Ok(())
    }
}
