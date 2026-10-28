# Requerimientos

## Objetivo general

Permitir al usuario controlar sus finanzas y conocer su dinero entre gastos, ahorros y deudas.

El contenido de RF-15 se conserva como objetivo general porque expresa el propósito del sistema, sin definir un comportamiento verificable por sí solo. RF-15 deja de ser un requisito funcional y su identificador queda reservado para mantener la trazabilidad.

El sistema es de uso local.

## Requerimientos funcionales

|Id|Requerimiento|
|--|-------------|
|RF-01|El sistema debe exigir la identificación y autenticación del usuario para acceder a la información de las cuentas|
|RF-02|El sistema debe asociar cada cuenta a un usuario propietario y restringir el acceso de los usuarios a sus propias cuentas, con la excepción de consulta del administrador definida en RF-03|
|RF-03|El sistema debe permitir al administrador consultar toda la información de cualquier cuenta, sin permitirle crear, modificar ni eliminar información de las cuentas|
|RF-04|El sistema debe distinguir las cuentas según su tipo|
|RF-05|El sistema debe actualizar el saldo de dinero de cada cuenta sumando los importes de los movimientos de dinero de entrada y restando los de salida, con la misma regla para todos los tipos de cuenta|
|RF-06|El sistema debe distinguir los movimientos de dinero de los movimientos de deuda y asociar cada movimiento a su cuenta y al saldo sobre el que opera|
|RF-07|El sistema debe generar dos movimientos de dinero por cada transferencia entre cuentas: uno de salida en la cuenta de origen y otro de entrada en la cuenta de destino|
|RF-08|El sistema debe representar las deudas mediante cuentas de tipo deuda|
|RF-09|El sistema debe representar las tarjetas de crédito como un tipo de cuenta|
|RF-10|El sistema debe permitir registrar una compra en una tarjeta de crédito indicando la cantidad de cuotas y el mes del primer pago, y generar las cuotas con sus importes calculados|
|RF-11|El sistema debe registrar en auditoría la creación y cada modificación de los movimientos de dinero y de deuda|
|RF-12|El sistema debe registrar una fecha para cada movimiento y permitir buscar movimientos por esa fecha|
|RF-13|El sistema debe registrar los eventos y comportamientos de la aplicación mediante logs|
|RF-14|El sistema debe permitir asignar y modificar el orden de los movimientos para su presentación en listados|
|RF-16|El sistema debe permitir pagar una cuota pendiente de una cuenta de deuda o tarjeta de crédito indicando la fecha y la cuenta pagadora, generar dos movimientos vinculados que resten el importe al saldo de dinero de la cuenta pagadora y al saldo de deuda de la cuenta de deuda o tarjeta de crédito, y retirar la cuota de los pendientes|
|RF-17|El sistema debe permitir consultar las cuotas pendientes y el importe total a pagar por ellas en cada uno de los próximos meses|
|RF-18|El sistema debe permitir el registro de usuarios|
|RF-19|El sistema debe permitir a los usuarios autenticados cerrar su sesión|
|RF-20|El sistema debe permitir a cada usuario consultar la información de su propio perfil|
|RF-21|El sistema debe permitir al usuario crear cuentas de los tipos admitidos y asociarlas a su propiedad|
|RF-22|El sistema debe permitir al usuario consultar sus propias cuentas filtrándolas por tipo|
|RF-23|El sistema debe permitir al usuario modificar la información de sus propias cuentas|
|RF-24|El sistema debe permitir al usuario eliminar sus propias cuentas junto con toda su información asociada|
|RF-25|El sistema debe permitir al usuario modificar movimientos de dinero o de deuda de sus propias cuentas, ajustar el saldo afectado por la diferencia entre el efecto anterior y el nuevo del movimiento, y conservar los cambios en la auditoría|
|RF-26|El sistema debe permitir al administrador consultar los logs de la aplicación|
|RF-27|El sistema debe mantener un saldo de deuda separado del saldo de dinero en las cuentas de tipo deuda y tarjeta de crédito|
|RF-28|El sistema debe permitir registrar la adquisición de una deuda con su fecha y cuotas pendientes, generar un movimiento de deuda que incremente el saldo de deuda por el importe adquirido y conservar las cuotas pendientes hasta su pago|

## Consideraciones por requerimiento

Se incluyen únicamente los requisitos que necesitan explicaciones adicionales. Todas las consideraciones de esta tabla han sido confirmadas por el usuario.

|Requerimiento|Consideraciones|
|--|--|
|RF-03|La consulta del administrador comprende toda la información asociada a una cuenta, incluidos sus movimientos y cuotas. Su acceso es de solo lectura.|
|RF-10|Al registrar la compra se debe indicar el mes y año de pago de la primera cuota. El valor por defecto debe ser el mes siguiente al de la fecha de compra ingresada; el usuario puede cambiarlo antes de registrar la compra. Por ejemplo, para una compra con fecha de diciembre de 2026, el valor por defecto es enero de 2027.|
|RF-10 y RF-17|Las cuotas corresponden a meses consecutivos a partir del mes del primer pago. Los importes pendientes se agrupan según el mes y año de pago de cada cuota.|
|RF-10|Calcular los importes de las cuotas en pesos enteros, sin decimales. Dividir el total entre la cantidad de cuotas tomando la parte entera del cociente y repartir el resto, un peso por cuota, comenzando por las primeras, para que la suma coincida con el total de la compra. Por ejemplo, una compra de 100 pesos en tres cuotas genera importes de 34, 33 y 33 pesos.|
|RF-11|Registrar fecha y hora de la acción, usuario o proceso responsable, cuenta y movimiento involucrados, acción realizada y resultado. Cuando la acción cambie datos de un movimiento, conservar los valores anteriores y posteriores de los campos afectados.|
|RF-13|Registrar el inicio y cierre de la aplicación, los intentos de autenticación y accesos denegados, los errores y las operaciones completadas o fallidas. Cada entrada debe incluir fecha y hora, nivel de severidad, evento y resultado; cuando corresponda, un identificador que permita relacionar entradas de la misma operación y detalles técnicos del error. Excluir contraseñas, tokens, credenciales y el contenido financiero de cuentas, compras y movimientos.|
|RF-16 y RF-17|Una cuota pagada deja de formar parte de los pendientes y sus importes futuros. Si se paga una de seis cuotas pendientes, quedan cinco. Los dos movimientos vinculados registran el mismo importe y la fecha de pago indicada.|
|RF-10, RF-16 y RF-28|Las cuotas son registros independientes de los movimientos de dinero y de deuda. Se registran al adquirir la deuda con su fecha de adquisición y permanecen pendientes hasta pagarse. El pago genera un movimiento de dinero y uno de deuda vinculados.|
|RF-16|Al registrar el pago se propone por defecto la fecha del día en que se paga la cuota. El usuario puede cambiarla antes de registrar los dos movimientos. Debe ser propietario tanto de la cuenta pagadora como de la cuenta de deuda o tarjeta de crédito.|
|RF-24|La eliminación comprende la cuenta y todos sus registros asociados, incluidos movimientos, compras, cuotas y auditorías vinculadas a ella.|
|RF-25|El ajuste se calcula sobre el saldo afectado: saldo nuevo = saldo actual − efecto anterior + efecto nuevo. En movimientos de dinero, una entrada suma y una salida resta; en movimientos de deuda, un aumento suma y una disminución resta. Si el efecto se conserva, el saldo no cambia.|
|RF-06 y RF-27|Cada movimiento identifica si opera sobre el saldo de dinero o el saldo de deuda; ambos saldos se mantienen por separado.|
|RF-10 y RF-28|La compra a crédito registra un movimiento de deuda por el total de la compra, con su fecha de adquisición, y genera las cuotas pendientes. Ese registro aumenta el saldo de deuda y no modifica el saldo de dinero.|
|RF-16 y RF-27|Pagar una cuota de 50 pesos resta 50 al saldo de dinero de la cuenta pagadora y resta 50 al saldo de deuda de la cuenta de deuda o tarjeta de crédito. Cada efecto queda respaldado por su movimiento vinculado.|

## Trazabilidad

- Los requerimientos de cada caso de uso se indican en su archivo, accesible desde [use-cases.md](use-cases.md).
- RF-12 y RF-14 fueron confirmados expresamente por el usuario como parte del alcance.
- RF-10, RF-16, RF-17, RF-27 y RF-28 recogen la separación entre dinero y deuda, la adquisición de deudas con cuotas pendientes y los pagos mediante movimientos vinculados.
- El usuario confirmó las consideraciones de distribución de importes, auditoría y logs, precisando que los importes de las cuotas se calculan en pesos enteros.
- RF-02 y RF-03 recogen la propiedad de las cuentas y la excepción de consulta del administrador. La restricción del administrador a solo lectura procede del texto original.
- Todos los requisitos se clasifican como funcionales porque describen comportamientos o reglas del sistema. No se han definido atributos de calidad medibles que justifiquen requisitos no funcionales adicionales.
