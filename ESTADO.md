# Estado de Pondera

**Fecha:** 2026-09-27

**Fase actual:** modelado de los datos que Pondera debe conservar; el proyecto
binario de Cargo ya está creado.

## Decisiones y hechos comprobados

- Pondera será un proyecto de aprendizaje de Rust para trabajar con
  calificaciones, criterios de evaluación y competencias en el contexto de la
  LOMLOE.
- El usuario escribirá el código del proyecto. A petición expresa, el agente
  generó únicamente el andamiaje inicial de Cargo.
- La elección entre TUI y GUI sigue abierta. La hoja de cálculo servirá como
  referencia inicial para contrastar los cálculos.
- `Cargo.toml` declara un paquete binario llamado `pondera`, sin dependencias.
  `src/main.rs` contiene solo el programa de ejemplo generado por Cargo y
  `.gitignore` excluye `target`. Cargo generó también `Cargo.lock`.
- Pasan `cargo fmt --check`, `cargo check`, `cargo test` (aún sin pruebas) y
  `cargo clippy`.
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
  medias. Aún no se ha elegido un formato de almacenamiento.
- Se propuso identificar cada PR por evaluación y número de prueba, registrar
  su fecha y guardar una nota con dos decimales y una anotación opcional por
  alumno. Se explicó la conveniencia de separar los datos compartidos de la
  prueba de cada resultado individual. Esa separación aún no se ha acordado ni
  implementado; tampoco se ha definido cómo identificar al alumno.

## Siguiente paso

Confirmar la separación entre los datos compartidos de una PR y el resultado
de cada alumno, usando un único ejemplo ficticio.

## Fuera del alcance actual

- Elegir e implementar la interfaz.
- Añadir métricas adicionales antes de definir y comprobar el cálculo básico.
- Recuperaciones y casos excepcionales de redondeo.

## Registro de sesiones

- 2026-09-27: se adaptaron las instrucciones, se creó el proyecto de Cargo y
  se revisaron las reglas de la tabla de 2.º de ESO. Quedó pendiente la
  respuesta del departamento sobre el cálculo final. La sesión terminó al
  empezar a definir los datos de una PR y de cada alumno.
