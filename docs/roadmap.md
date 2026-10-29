# Roadmap

## T-01: Registro, acceso y primera entrada de dinero con persistencia

- Objetivo: registrar un usuario, iniciar sesión, crear una cuenta propia, registrar una entrada de dinero y recuperar el mismo saldo después de cerrar y volver a ejecutar la aplicación.
- Rama: `main`.
- Interfaz: terminal interactiva.
- Estado: en curso. El workspace y la entidad `User` con sus validaciones ya existen; falta completar el recorrido desde la terminal y guardar los datos.
- Crates: [README](../README.md#crates-del-proyecto).
- Historial: [roadmap-history.md](roadmap-history.md).

### Alcance

El recorrido incluye [registro](use-cases/CU-01.md), [inicio de sesión](use-cases/CU-02.md), [creación de cuenta](use-cases/CU-06.md), la entrada de dinero de [CU-12](use-cases/CU-12.md) y [consulta del saldo](use-cases/CU-11.md).

Se conservan las reglas de acceso, propiedad, tipo de cuenta, saldo, fecha, auditoría y logs de RF-01, RF-02, RF-04, RF-05, RF-06, RF-11, RF-12, RF-13, RF-18 y RF-21, en [requirements.md](requirements.md). Para T-01 se implementa solo lo que corresponde a este recorrido.

Salidas, transferencias, deudas, tarjetas, cuotas, modificaciones, eliminaciones y funciones administrativas quedan para otras tareas.

### Decisiones para T-01

- Registro con nombre de usuario único y contraseña protegida. Reutilizar las validaciones de `User` y mantener la sesión en memoria; al reiniciar se inicia sesión otra vez.
- Cuentas de tipo `efectivo` o `bancaria`, con saldo inicial cero. Las entradas usan importes positivos en pesos enteros y una fecha válida; comprobar que la suma no desborde.
- Persistencia en `data/arca-finance.sqlite3`, relativo al directorio de ejecución. Usar el mismo archivo tras reiniciar, excluirlo de Git junto con sus auxiliares y no reemplazarlo si no se puede leer. Guardar entrada, saldo y auditoría juntos antes de anunciar éxito; al cargar, leer el saldo sin volver a sumar entradas.

### Hitos

- [x] **H1. Establecer la base.** Workspace con ejecutable, core y almacenamiento; definición mínima de datos y entidad `User` con sus validaciones. Comprobación: la estructura y `User` están presentes, con pruebas ejecutadas en la implementación de la entidad.
- [ ] **H2. Registrar e iniciar sesión.** Conectar `User` con la terminal, proteger la contraseña y guardar el usuario. Comprobación: un usuario registrado puede iniciar sesión, incluso tras reiniciar; las credenciales incorrectas se rechazan.
- [ ] **H3. Crear una cuenta y registrar una entrada.** Guardar una cuenta del usuario autenticado con saldo cero; registrar importe y fecha, actualizar el saldo y conservar la auditoría de creación. Incorporar los logs del recorrido. Comprobación: una entrada de 1000 pesos deja el saldo en 1000; otro usuario no puede acceder a esa cuenta y un fallo no deja la entrada guardada parcialmente.
- [ ] **H4. Comprobar el recorrido completo.** Cerrar la aplicación y abrirla en otro proceso con el mismo almacenamiento. Comprobación: se recuperan usuario, cuenta, movimiento y auditoría, con el mismo saldo y sin duplicaciones. Ejecutar `cargo build`, `cargo test` y el recorrido manual, registrando sus resultados.

### Criterios de cierre

1. Completar desde la terminal el registro, acceso, creación de cuenta, entrada y consulta del saldo.
2. Recuperar los mismos datos y saldo tras reiniciar e iniciar sesión otra vez.
3. Rechazar credenciales incorrectas y acceso a cuentas ajenas, sin dejar cambios parciales.
4. Conservar la auditoría de la entrada y los logs exigidos, sin credenciales ni contenido financiero en los logs.
5. Completar las comprobaciones de los hitos. T-01 sigue pendiente en el historial hasta cumplirlas.

Próximo hito: H2, conectar el registro y el inicio de sesión con la terminal y el almacenamiento.
