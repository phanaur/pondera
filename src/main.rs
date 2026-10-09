use rusqlite::{Connection, Result};

fn main() {
    let path: String = String::from("/tmp/pondera-prueba.sqlite");
    let db = Connection::open(path);
}
