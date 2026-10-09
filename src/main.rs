use rusqlite::Connection;

fn main() {
    let path: String = String::from("/tmp/pondera-prueba.sqlite");
    let db = Connection::open(path);

    match db {
        Ok(conexion) => {
            println!("Base abierta");
            let resultado = conexion.execute(
                "CREATE TABLE alumnos (
              numero INTEGER PRIMARY KEY,
              nombre TEXT,
              apellidos TEXT,
              curso TEXT
            );",
                [],
            );

            match resultado {
                Ok(_) => println!("Tabla creada"),
                Err(resultado) => println!("La tabla no ha podido ser creada. Error {}", resultado),
            }
        }
        Err(error) => println!("No se ha podido abrir la base de datos. Error: {}", error),
    }
}
