# Estado de Pondera

**Fecha:** 2026-10-09

**Fase actual:** apertura de SQLite y creación de la primera tabla de alumnos
desde Rust, con comprobación de los resultados de ambas operaciones.

## Decisiones y hechos comprobados

- Pondera será un proyecto de aprendizaje de Rust para trabajar con
  calificaciones, criterios de evaluación y competencias en el contexto de la
  LOMLOE.
- El usuario escribirá el código del proyecto. A petición expresa, el agente
  generó únicamente el andamiaje inicial de Cargo.
- La elección entre TUI y GUI sigue abierta. La hoja de cálculo servirá como
  referencia inicial para contrastar los cálculos.
- `Cargo.toml` declara el paquete binario `pondera` y la dependencia
  `rusqlite` 0.40.2 con la opción `bundled`; `Cargo.lock` se actualizó.
  `.gitignore` excluye `target`.
- `src/main.rs` abre `/tmp/pondera-prueba.sqlite` desde `main` y examina el
  `Result` de `Connection::open` mediante `match`. Si la apertura funciona,
  envía una instrucción `CREATE TABLE alumnos` mediante `execute` y examina
  también su resultado. Ambas operaciones muestran un mensaje de éxito o el
  error recibido.
- El usuario ejecutó `cargo run` y obtuvo «Base abierta» y «Tabla creada».
  Una inspección posterior en modo de lectura confirmó que la base contiene
  la tabla `alumnos` con `numero INTEGER PRIMARY KEY`, `nombre TEXT`,
  `apellidos TEXT` y `curso TEXT`. Aún no se ha implementado la inserción de
  alumnos ni se ha comprobado guardar y recuperar sus datos.
- En el código final pasan `cargo fmt --check` y `cargo check`, sin
  advertencias. El usuario sustituyó por `Ok(_)` la captura del número de
  filas modificadas que no utilizaba. Las pruebas y Clippy pasaron en la
  revisión anterior; no se han repetido para este cambio. Aquella ejecución
  de `cargo test` tenía cero pruebas.
- La instrucción actual utiliza `CREATE TABLE` sin `IF NOT EXISTS`: si la
  tabla ya existe, la creación devuelve un error que el programa muestra.
- `README.md` presenta el proyecto. Aún no se ha implementado ninguna función
  de calificaciones.
- Se trabajará solo con datos ficticios.
- Un instrumento de evaluación puede evaluar varios criterios y asignar a cada
  uno un peso dentro de ese instrumento. Un mismo criterio puede aparecer en
  varios instrumentos con pesos iguales o diferentes.
- En el ejemplo aportado por el usuario, el 60 % y el 40 % distribuyen la nota
  de un ejercicio entre resolución matemática y descripción del proceso.
- Según la regla indicada por el usuario, el peso acumulado de un criterio en
  la asignatura es la suma de sus pesos en cada instrumento de evaluación.
- En la tabla de 2.º de ESO aportada, los pesos finales de los criterios suman
  100 % y los de las competencias también. Los porcentajes PR, SA, CC y DP de
  cada criterio suman aproximadamente 100 %; algunos valores están redondeados.
- El usuario confirmó que esos porcentajes distribuyen el peso final de cada
  criterio entre instrumentos. Para el criterio 1.1, aproximadamente 10 puntos
  proceden de PR y 0,5 de CC. Los instrumentos aportan aproximadamente 60, 20,
  10 y 10 puntos al total, respectivamente.
- Se registra una nota global por cada PR realizada en una evaluación. En el
  caso habitual hay dos PR por evaluación. Sus notas se promedian
  aritméticamente para obtener la nota de PR en esa evaluación.
- Con una nota media por instrumento, la nota de cada criterio en la evaluación
  se obtiene ponderando esas medias con los porcentajes de la fila del criterio.
- El fragmento de la programación didáctica aportado por el usuario establece
  que la calificación de cada evaluación se obtiene de las calificaciones de
  los criterios multiplicadas por sus pesos. La calificación final de la materia
  es la media aritmética de las tres calificaciones de evaluación.
- La programación indica que la calificación final consignada es un entero,
  generalmente obtenido truncando esa media, con decisiones particulares cerca
  del entero siguiente. Aún no se ha aclarado si las calificaciones de
  evaluación se promedian con decimales o una vez convertidas en enteros. El
  usuario ha consultado al departamento cómo se calcula la evaluación final;
  la respuesta está pendiente.
- El usuario quiere abordar el registro, lectura y modificación de
  calificaciones y ponderaciones antes de seguir con la implementación de
  medias. Se ha elegido SQLite como formato de almacenamiento.
- Se propuso identificar cada PR por evaluación y número de prueba, registrar
  su fecha y guardar una nota con dos decimales y una anotación opcional por
  alumno. Los campos definitivos siguen pendientes.
- Se acordó separar los datos de cada alumno, los datos compartidos de cada PR
  y el resultado de un alumno en esa PR. Los datos del alumno son comunes a
  todos los instrumentos: con dos PR se guardan una vez y hay dos resultados.
  La primera tabla de alumnos utiliza un identificador entero como clave
  primaria. Las tablas de pruebas y resultados siguen pendientes.
- Los datos iniciales del alumno son nombre, apellidos y curso. El curso
  representa el nivel educativo, como 2.º de ESO, y se guarda como texto.
- Un mismo alumno puede cursar Matemáticas y Conocimiento de las Matemáticas.
  Se explicó un diseño con una tabla de asignaturas y otra de matrículas que
  relaciona alumnos con asignaturas; esas dos tablas aún no se han implementado.
- El usuario eligió SQLite tras comparar su uso con archivos CSV y decidió
  empezar con acceso directo mediante SQL. SQLite no necesita un servidor
  aparte. Ha añadido `rusqlite` como biblioteca de acceso desde Rust.

## Siguiente paso

Explicar `CREATE TABLE IF NOT EXISTS` para que el usuario adapte la creación
de `alumnos` y compruebe que puede ejecutar el programa dos veces sin error
por la existencia previa de la tabla.

## Fuera del alcance actual

- Elegir e implementar la interfaz.
- Añadir métricas adicionales antes de definir y comprobar el cálculo básico.
- Recuperaciones y casos excepcionales de redondeo.
- Considerar una ORM si el acceso directo resulta repetitivo más adelante.

## Registro de sesiones

- 2026-09-27: se adaptaron las instrucciones, se creó el proyecto de Cargo y
  se revisaron las reglas de la tabla de 2.º de ESO. Quedó pendiente la
  respuesta del departamento sobre el cálculo final. La sesión terminó al
  empezar a definir los datos de una PR y de cada alumno.
- 2026-10-01: se confirmó con un ejemplo ficticio que los datos de un alumno
  se guardan una vez y cada PR realizada produce un resultado propio. El
  usuario aclaró que su pregunta principal era cómo guardar los datos; la
  comparación con CSV llevó a elegir SQLite. Después se eligió el acceso
  directo mediante SQL para la primera versión. El usuario añadió `rusqlite`
  y una primera función de apertura; la compilación de pruebas reveló que
  falta la biblioteca SQLite necesaria para enlazar en este entorno.
- 2026-10-09: el usuario activó `bundled` en `rusqlite`; `cargo test` enlaza y
  pasa. Se descartó la función auxiliar `open_db`; el usuario abrió un archivo
  de prueba desde `main`, aunque aún se ignora el resultado de la operación.
  Se acordó retomar las explicaciones de SQL y `rusqlite` desde cero antes de
  continuar con la implementación.
- 2026-10-09 (segunda sesión): se explicó la separación entre alumnos y
  matrículas, la estructura de una tabla y la instrucción `CREATE TABLE`.
  El usuario escribió la definición de `alumnos` y la ejecutó desde Rust,
  comprobando apertura y creación con `match`. Se trabajó el valor de la
  última expresión de un bloque y el uso de `Ok(_)`. La ejecución y la
  inspección de SQLite confirmaron la creación; el código final pasa formato
  y compilación sin advertencias. Queda retomar la creación cuando la tabla
  ya existe.
