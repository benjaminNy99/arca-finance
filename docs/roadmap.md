# Roadmap

## T-01: Registro, acceso y primera entrada de dinero con persistencia

- Objetivo: registrar un usuario, iniciar sesión, crear una cuenta financiera propia, registrar una entrada de dinero y consultar el mismo saldo después de cerrar y volver a ejecutar la aplicación.
- Interfaz prevista: terminal interactiva, conforme al [README](../README.md).
- Historial: [roadmap-history.md](roadmap-history.md).

### Alcance y trazabilidad

El recorrido comprende [registrar usuario](use-cases/CU-01.md), [iniciar sesión](use-cases/CU-02.md), [crear cuenta financiera](use-cases/CU-06.md), la entrada de dinero de [CU-12](use-cases/CU-12.md) y [consultar saldos](use-cases/CU-11.md). La conservación de los datos después de reiniciar responde a lo solicitado para esta tarea.

Se aplican los requisitos de autenticación, propiedad de las cuentas, tipo de cuenta, actualización del saldo, fecha del movimiento, auditoría y logs que correspondan a estas operaciones: RF-01, RF-02, RF-04, RF-05, RF-06, RF-11, RF-12, RF-13, RF-18 y RF-21, recogidos en [requirements.md](requirements.md).

Esta tarea se limita al recorrido solicitado para un usuario propietario. Las salidas, transferencias, deudas, tarjetas, cuotas, modificaciones, eliminaciones y consultas administrativas quedan fuera de esta tarea.

### Crates y módulos de Rust

| Crate | Responsabilidad |
|---|---|
| `arca-finance` | Ejecutable e interfaz de terminal. Configura los logs y conecta los componentes. |
| `arca-core` | Reglas y operaciones de negocio, independientes de la terminal y del almacenamiento concreto. |
| `arca-storage` | Persistencia local de usuarios, cuentas, movimientos y auditorías. Guarda conjuntamente los cambios de una operación. |

`arca-finance` utiliza los otros dos crates. `arca-storage` utiliza los tipos y contratos mínimos de `arca-core`; `arca-core` no depende de los otros crates.

Los módulos evaluados para `arca-core` son:

| Módulo | Responsabilidad | Uso en T-01 |
|---|---|---|
| `users` | Identificación, autenticación y permisos. | Registro, inicio de sesión e identidad del usuario. |
| `accounts` | Cuentas, tipos, propietarios y saldos. | Creación de cuenta propia y consulta del saldo de dinero. |
| `transactions` | Movimientos, transferencias y ajustes de saldo. | Registro de una entrada de dinero y su efecto sobre el saldo. |
| `audit` | Registros de creación y modificación de movimientos. | Auditoría de creación de la entrada de dinero. |
| `debts` | Adquisiciones de deuda, compras, cuotas y pagos. | Se incorporará en una tarea posterior. |

Crear solo los módulos necesarios para T-01, sin implementar las operaciones futuras de cada módulo. Los logs se configuran en el ejecutable mediante `tracing`, ya declarado como dependencia.

### Hitos

Realizar los hitos en el orden indicado. Todas las casillas permanecen pendientes porque el recorrido aún no está implementado.

- [ ] **H1. Definir los datos mínimos y la persistencia local.** Resolver las decisiones pendientes y definir cómo se guardarán usuarios, cuentas, movimientos y auditoría, conservando sus relaciones. Comprobación: quedan documentados los campos, validaciones, saldo inicial, mecanismo de autenticación y almacenamiento necesarios para el recorrido, con la distribución de responsabilidades indicada en los crates y módulos.
- [ ] **H2. Preparar los crates y registrar usuarios desde la terminal.** Configurar un workspace conservando el ejecutable `arca-finance` e incorporando `arca-core` y `arca-storage`. Implementar el registro y guardar los datos con el mecanismo definido, sin almacenar contraseñas en texto plano si se utilizan. Comprobación: el workspace compila; registrar un usuario, cerrar la aplicación y verificar que sigue disponible al volver a ejecutarla.
- [ ] **H3. Iniciar sesión y controlar el acceso.** Autenticar al usuario registrado y exigir autenticación para crear cuentas, registrar entradas y consultar saldos. Comprobación: los datos válidos permiten acceder y los datos de autenticación incorrectos se rechazan sin habilitar operaciones protegidas.
- [ ] **H4. Crear una cuenta financiera propia.** Guardar la cuenta con su tipo, propietario y saldo inicial definido en H1. Comprobación: la cuenta queda asociada al usuario autenticado, sigue disponible después de reiniciar y otro usuario no puede consultar ni modificar sus datos.
- [ ] **H5. Registrar una entrada y consultar el saldo.** Guardar el movimiento con cuenta, importe, dirección de entrada y fecha; sumar su importe al saldo de dinero y conservar la auditoría exigida por RF-11. Comprobación: el saldo pasa de un valor inicial S a S + I al registrar una entrada de importe I; el movimiento y su auditoría quedan persistidos y no se confirma una operación guardada parcialmente.
- [ ] **H6. Registrar los eventos del recorrido.** Incorporar los logs de inicio y cierre, intentos de autenticación, accesos denegados y operaciones completadas o fallidas conforme a RF-13. Comprobación: los eventos incluyen fecha y hora, severidad, evento y resultado, sin credenciales ni contenido financiero; se incorporan datos de correlación o diagnóstico cuando corresponda.
- [ ] **H7. Verificar el recorrido completo tras reiniciar.** Ejecutar el escenario de cierre en dos procesos distintos. Comprobación: el segundo proceso recupera el usuario, la cuenta, la entrada y su auditoría; el saldo conserva el valor esperado y no se vuelve a sumar la entrada al cargar los datos. Ejecutar `cargo build` y `cargo test` con pruebas relevantes de persistencia, cálculo del saldo y control de acceso.

### Decisiones pendientes

- CU-01 y CU-02 no definen los campos obligatorios de registro ni el mecanismo de autenticación. Elegir la alternativa mínima para el uso local y definir sus validaciones antes de implementar H2 y H3.
- CU-06 deja pendientes los datos obligatorios, tipos admitidos y saldo inicial. Definir los necesarios para esta tarea sin incorporar el comportamiento de deudas o tarjetas.
- Elegir el almacenamiento local, su ubicación y cómo guardar de forma consistente el movimiento, el saldo y la auditoría. El repositorio aún no tiene persistencia; justificar cualquier dependencia nueva si resulta necesaria.
- Suposición para el recorrido: después de reiniciar, el usuario vuelve a iniciar sesión. La conservación automática de la sesión no forma parte del objetivo.

### Criterios de cierre

1. Desde la terminal, registrar un usuario, iniciar sesión y crear una cuenta propia con saldo inicial conocido S.
2. Registrar una entrada de dinero de importe I y fecha indicada; consultar un saldo de S + I y comprobar que existen el movimiento y su auditoría.
3. Cerrar por completo la aplicación y volver a ejecutarla usando el mismo almacenamiento local.
4. Iniciar sesión con el usuario registrado y consultar la misma cuenta: el saldo debe seguir siendo S + I, conservando el movimiento y su auditoría sin duplicaciones.
5. Comprobar que la autenticación incorrecta y los intentos de acceso a cuentas de otro usuario se rechazan, y que los logs del recorrido respetan RF-13.
6. Completar las comprobaciones de los hitos y registrar los resultados reales de `cargo build`, `cargo test` y la ejecución manual del recorrido. Marcar T-01 como terminada en ambos documentos solo cuando se cumplan estos criterios.

Próximo hito: H1, definir los datos mínimos y la persistencia local.
