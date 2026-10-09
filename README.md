# Pondera

Pondera es un proyecto personal de aprendizaje de Rust para gestionar
calificaciones, criterios de evaluación y competencias en el contexto de la
LOMLOE. La interfaz, TUI o GUI, aún no está decidida.

El programa abre una base de prueba SQLite en `/tmp/pondera-prueba.sqlite`
mediante `rusqlite` y crea una primera tabla de alumnos. Comprueba el resultado
de ambas operaciones y muestra un mensaje de éxito o el error recibido. Aún
no se han implementado la inserción de alumnos, el registro de notas ni su
cálculo. Durante el desarrollo solo se usarán datos ficticios de alumnos.

Consulta [ESTADO.md](ESTADO.md) para conocer las decisiones confirmadas, las
dudas pendientes y el siguiente paso. [AGENTS.md](AGENTS.md) describe la forma
de trabajo con el tutor.
