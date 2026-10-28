# Casos de uso

## Actores

- **Usuario**
- **Administrador**
- **Sistema**

Los casos desarrollados se encuentran en archivos individuales dentro de `docs/use-cases/`. Los objetivos compartidos se agrupan mediante flujos y las responsabilidades del sistema se incorporan a los pasos correspondientes.

## 1. Usuarios y acceso

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-01](use-cases/CU-01.md)|Registrar usuario|Usuario|
|[CU-02](use-cases/CU-02.md)|Iniciar sesión|Usuario/Administrador|
|[CU-03](use-cases/CU-03.md)|Cerrar sesión|Usuario/Administrador|
|[CU-04](use-cases/CU-04.md)|Consultar perfil de usuario|Usuario|
|[CU-05](use-cases/CU-05.md)|Consultar cuentas de todos los usuarios|Administrador|

## 2. Cuentas financieras

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-06](use-cases/CU-06.md)|Crear cuenta financiera|Usuario|
|[CU-07](use-cases/CU-07.md)|Consultar cuentas propias|Usuario|
|[CU-08](use-cases/CU-08.md)|Consultar la información de una cuenta|Usuario/Administrador|
|[CU-09](use-cases/CU-09.md)|Modificar información de una cuenta|Usuario|
|[CU-10](use-cases/CU-10.md)|Eliminar cuenta financiera|Usuario|
|[CU-11](use-cases/CU-11.md)|Consultar saldos de una cuenta|Usuario/Administrador|

## 3. Movimientos

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-12](use-cases/CU-12.md)|Registrar entrada o salida de dinero|Usuario|
|[CU-13](use-cases/CU-13.md)|Consultar movimientos de una cuenta|Usuario/Administrador|
|[CU-14](use-cases/CU-14.md)|Consultar detalle de un movimiento|Usuario/Administrador|
|[CU-15](use-cases/CU-15.md)|Corregir un movimiento registrado|Usuario|
|[CU-16](use-cases/CU-16.md)|Transferir dinero entre cuentas propias|Usuario|
|[CU-17](use-cases/CU-17.md)|Buscar movimientos por fecha|Usuario/Administrador|
|[CU-18](use-cases/CU-18.md)|Asignar o modificar el orden de presentación de los movimientos|Usuario|

## 4. Deudas, compras y cuotas

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-19](use-cases/CU-19.md)|Registrar compra con tarjeta de crédito|Usuario|
|[CU-20](use-cases/CU-20.md)|Pagar una cuota de deuda|Usuario|
|[CU-21](use-cases/CU-21.md)|Consultar cuotas pendientes e importes por mes|Usuario/Administrador|
|[CU-24](use-cases/CU-24.md)|Registrar adquisición de deuda|Usuario|

## 5. Auditoría

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-22](use-cases/CU-22.md)|Consultar auditoría de una cuenta|Administrador|

## 6. Logs del sistema

|ID|Caso de uso|Actor principal|
|--|-----------|---------------|
|[CU-23](use-cases/CU-23.md)|Consultar logs de aplicación|Administrador|

## Objetivos agrupados

Los siguientes objetivos se desarrollan dentro de los casos y flujos indicados.

|Objetivo|Ubicación|
|--|--|
|Consultar cuentas por tipo|[CU-07](use-cases/CU-07.md), CU-07.A1|
|Registrar egreso de dinero|[CU-12](use-cases/CU-12.md), CU-12.A1|
|Consultar movimientos de una transferencia|[CU-13](use-cases/CU-13.md), CU-13.A1|
|Registrar cuenta de deuda|[CU-06](use-cases/CU-06.md), flujo principal con tipo deuda|
|Registrar aumento de deuda|[CU-24](use-cases/CU-24.md), flujo principal|
|Registrar pago de deuda|[CU-20](use-cases/CU-20.md), flujo principal|
|Consultar saldo de una cuenta de deuda|[CU-11](use-cases/CU-11.md), flujo principal con cuenta de deuda|
|Registrar tarjeta de crédito|[CU-06](use-cases/CU-06.md), flujo principal con tipo tarjeta de crédito|
|Consultar saldo de una tarjeta de crédito|[CU-11](use-cases/CU-11.md), flujo principal con tarjeta de crédito|
|Consultar cuotas pendientes|[CU-21](use-cases/CU-21.md), flujo principal|
|Consultar detalle de una cuota pendiente|[CU-21](use-cases/CU-21.md), CU-21.A1|
|Consultar auditoría de un movimiento|[CU-22](use-cases/CU-22.md), CU-22.A1|

## Comportamientos compartidos

- **Acceso:** antes de consultar o modificar información de cuentas se valida la autenticación y la propiedad de las cuentas. El administrador puede consultar cualquier cuenta con acceso de solo lectura; se rechazan sus operaciones de escritura.
- **Saldos:** cada cuenta tiene un saldo de dinero; las cuentas de deuda y tarjetas de crédito tienen además un saldo de deuda separado. Las entradas de dinero suman y las salidas restan en el saldo de dinero; los aumentos y disminuciones de deuda operan sobre el saldo de deuda. Cada cambio está respaldado por un movimiento de la clase correspondiente. Al modificar un movimiento se ajusta el saldo afectado por la diferencia entre su efecto anterior y el nuevo.
- **Adquisición y pago:** adquirir una deuda aumenta su saldo mediante un movimiento de deuda y registra las cuotas pendientes. Pagar una cuota genera dos movimientos vinculados: salida de dinero en la cuenta pagadora y disminución de deuda en la cuenta correspondiente.
- **Auditoría:** se auditan la creación y cada modificación de los movimientos de dinero y de deuda. Cada registro de auditoría conserva fecha y hora de la acción, usuario o proceso responsable, cuenta y movimiento involucrados, acción y resultado. Cuando se modifican datos de un movimiento se conservan los valores anteriores y posteriores de los campos afectados.
- **Logs:** se registran el inicio y cierre de la aplicación, los intentos de autenticación, accesos denegados, errores y operaciones completadas o fallidas. Cada entrada incluye fecha y hora, severidad, evento y resultado, y datos de correlación o diagnóstico cuando correspondan. Se excluyen contraseñas, tokens, credenciales y contenido financiero de cuentas, compras y movimientos.

Las responsabilidades del actor Sistema se desarrollan en los pasos y reglas compartidas indicados a continuación.

|Comportamiento|Actor|Ubicación|
|--|--|--|
|Validar autenticación y permisos de acceso|Sistema|CU-02 y validaciones de acceso de los casos de cuenta|
|Actualizar el saldo afectado por cada movimiento|Sistema|Pasos de registro o modificación de movimientos de dinero o deuda y reglas compartidas de saldos|
|Generar cuotas de una compra|Sistema|[CU-19](use-cases/CU-19.md), pasos CU-19.6 y CU-19.7|
|Registrar adquisición de deuda y sus cuotas|Sistema|[CU-24](use-cases/CU-24.md), pasos CU-24.4 a CU-24.6|
|Registrar los dos movimientos vinculados de un pago|Sistema|[CU-20](use-cases/CU-20.md), pasos CU-20.7 a CU-20.9|
|Registrar auditoría de un movimiento|Sistema|Pasos de registro o modificación de movimientos y regla compartida de auditoría|
|Registrar eventos de la aplicación|Sistema|Pasos y regla compartida de logs|
