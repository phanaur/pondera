use rusqlite::{Connection, Result};

fn main() {
    println!("Hello, world!");
}

fn open_db(path: String) -> Result<()> {
    let db = Connection::open(path);

    Ok(())
}
