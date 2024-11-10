use rusqlite::{self, Connection};
use std::fs;

enum MyError {}

trait AbstractCommand {
    fn get_name(&self) -> Option<&str>;
    fn exec(&mut self, args: &str) -> Result<(), MyError>;
}

struct BkCommand {}
struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: i32,
}
struct HelpCommand {}

impl AbstractCommand for BkCommand {
    fn get_name(&self) -> Option<&str> {
        Some("bk")
    }

    fn exec(&mut self, args: &str) -> Result<(), MyError> {
        if let Some(tp) = args.split_whitespace().next() {
            match tp {
                "add" => {
                    Self::add(args);
                }
                "search" => {
                    Self::search(args.split_whitespace().nth(1).unwrap_or(" "));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

struct Bookmark {
    name: String,
    url: String,
}

impl BkCommand {
    fn create_table_bookmarks() {
        if let Ok(conn) = Connection::open("bookmarks.db") {
            let create = r"
            create table if not exists bookmarks (
                name text    not null,
                url text not null
            );
            ";
            let _ = conn.execute(create, ());
            let _ = conn.close();
        }
    }

    fn insert(bookmark: (&str, &str)) {
        let name = bookmark.0;
        let url = bookmark.1;
        println!("Terminal: Inserting {} {}", name, url);
        if let Ok(conn) = Connection::open("bookmarks.db") {
            let _ = conn.execute(
                "insert into bookmarks (name, url) values (?1, ?2);",
                (name, url),
            );
            let _ = conn.close();
        }
    }

    fn add(args: &str) {
        let mut it = args.split_whitespace();
        it.next();
        if let (Some(name), Some(url)) = (it.next(), it.next()) {
            Self::create_table_bookmarks();
            Self::insert((name, url));
        }
    }

    fn search(word: &str) {
        if let Ok(conn) = Connection::open("bookmarks.db") {
            if let Ok(mut stmt) = conn.prepare("select * from bookmarks") {
                let bookmark_iter = stmt.query_map([], |row| {
                    Ok(Bookmark {
                        name: row.get("name").unwrap_or(String::from(" ")),
                        url: row.get("url").unwrap_or(String::from(" ")),
                    })
                });
                if let Ok(it) = bookmark_iter {
                    for b in it {
                        if let Ok(b) = b {
                            if b.name.contains(word) {
                                println!("{}", b.name);
                            }
                            if b.url.contains(word) {
                                println!("{}", b.url);
                            }
                        }
                    }
                }
            }
            let _ = conn.close();
        }
    }
}

impl AbstractCommand for PingCommand {
    fn get_name(&self) -> Option<&str> {
        Some("ping")
    }

    fn exec(&mut self, _: &str) -> Result<(), MyError> {
        println!("Terminal: pong");
        Ok(())
    }
}

impl AbstractCommand for CountCommand {
    fn get_name(&self) -> Option<&str> {
        Some("count")
    }
    fn exec(&mut self, args: &str) -> Result<(), MyError> {
        let mut count: i32 = 0;
        for _ in String::from(args).split_whitespace() {
            count = count + 1;
        }
        println!("Terminal: Command count received {} arguments", count);
        Ok(())
    }
}

impl AbstractCommand for HelpCommand {
    fn get_name(&self) -> Option<&str> {
        Some("help")
    }
    fn exec(&mut self, _: &str) -> Result<(), MyError> {
        println!("Terminal: The available commands are: ping, count [args], times, help");
        println!("Terminal: ping: prints pong!");
        println!("Terminal: count: prints the number of arguments it received; count a b c -> will print counted 3 args");
        println!("Terminal: times: prints how many times this command (only this command!) has been called");
        println!("Terminal: help: prints the available commands and what they do");
        Ok(())
    }
}

impl AbstractCommand for TimesCommand {
    fn get_name(&self) -> Option<&str> {
        Some("times")
    }

    fn exec(&mut self, _: &str) -> Result<(), MyError> {
        self.count = self.count + 1;
        println!(
            "Terminal: The times command was called {} times!",
            self.count
        );
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn AbstractCommand>>,
}

impl Terminal {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn AbstractCommand>) -> Result<(), MyError> {
        self.commands.push(command);
        Ok(())
    }

    fn run(&mut self) -> Result<(), MyError> {
        let s = fs::read_to_string("file.txt").ok().unwrap();
        'outer: for line in s.lines() {
            let mut it = line.split_whitespace();
            if let Some(command) = it.next() {
                let mut args: String = String::new();
                for arg in it {
                    args.push_str(&format!("{arg} "));
                }
                let mut ok: bool = false;
                for c in &mut self.commands {
                    if c.get_name().unwrap_or(" ") == command {
                        let result = c.exec(&args);
                        match result {
                            Ok(_) => {
                                ok = true;
                                println!("Terminal: Command {} ran succesfully", command);
                                break;
                            }
                            Err(_) => {
                                println!("Terminal: Error!");
                            }
                        }
                    }
                }

                if command == "stop" {
                    break 'outer;
                }

                if !ok {
                    println!(
                        "Terminal: Invalid command! Try one of the following: ping, times, count"
                    );
                }
            } else {
                println!("Terminal: Empty line!");
            }
        }

        Ok(())
    }
}

fn main() {
    let mut terminal = Terminal::new();

    let _ = terminal.register(Box::new(PingCommand {}));
    let _ = terminal.register(Box::new(CountCommand {}));
    let _ = terminal.register(Box::new(TimesCommand { count: 0 }));
    let _ = terminal.register(Box::new(HelpCommand {}));
    let _ = terminal.register(Box::new(BkCommand {}));
    let _ = terminal.run();
}
