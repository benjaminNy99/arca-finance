# Reglas de negocio

|ID|Regla de negocio|
|--|-----------|
|RN-01|Una cuenta pertenece a un usuario|
|RN-02|El acceso de un usuario se restringe a sus propias cuentas, con la excepción de consulta del administrador definida en RN-03|
|RN-03|El administrador puede consultar toda la información de cualquier cuenta, sin crear, modificar ni eliminar información de las cuentas|
|RN-04|Una transferencia genera exactamente dos movimientos de dinero: una salida en la cuenta de origen y una entrada en la cuenta de destino|
|RN-05|Cada movimiento se asocia a una cuenta e identifica si opera sobre su saldo de dinero o su saldo de deuda|
|RN-06|Al registrar una compra en tarjeta de crédito se indica la cantidad de cuotas y el mes y año del primer pago, se generan las cuotas y se registra un movimiento de deuda que aumenta el saldo de deuda por el total de la compra|
|RN-07|El mes del primer pago tiene como valor por defecto el mes siguiente al de la fecha de compra ingresada y puede cambiarse antes de registrar la compra|
|RN-08|Las cuotas corresponden a meses consecutivos a partir del mes y año del primer pago|
|RN-09|Los importes de las cuotas se calculan en pesos enteros: se toma la parte entera de la división del total entre la cantidad de cuotas y se reparte el resto, un peso por cuota, desde las primeras, conservando el total de la compra|
|RN-10|Pagar una cuota genera dos movimientos vinculados por el mismo importe: una salida de dinero en la cuenta pagadora y una disminución de deuda en la cuenta de deuda o tarjeta de crédito; la cuota deja de estar pendiente|
|RN-11|Los importes a pagar por mes se obtienen de las cuotas pendientes, agrupadas por mes y año de pago|
|RN-12|Cada cuenta tiene un tipo y un saldo de dinero; las cuentas de deuda y las tarjetas de crédito tienen además un saldo de deuda independiente|
|RN-13|Cada movimiento tiene una fecha por la que puede buscarse|
|RN-14|El orden de presentación de los movimientos en listados puede asignarse y modificarse|
|RN-15|En todos los tipos de cuenta, las entradas de dinero suman al saldo de dinero y las salidas lo restan; los aumentos de deuda suman al saldo de deuda y las disminuciones lo restan|
|RN-16|Al modificar un movimiento de dinero o deuda, el saldo que afecta se ajusta restando su efecto anterior y sumando el nuevo; si el efecto se conserva, ese saldo no cambia|
|RN-17|La creación y cada modificación de movimientos de dinero o deuda se registran en auditoría; los cambios conservan los valores anteriores y posteriores|
|RN-18|El propietario puede eliminar una cuenta junto con toda su información asociada, incluidos movimientos, compras, cuotas y auditorías vinculadas a ella|
|RN-19|Al pagar una cuota, el usuario puede indicar la fecha de sus dos movimientos vinculados; por defecto se propone la fecha del día del pago|
|RN-20|Las cuotas son registros independientes de los movimientos; se registran con la fecha en que se adquiere la deuda y permanecen pendientes hasta su pago|
|RN-21|Adquirir una deuda genera un movimiento de deuda por el importe adquirido, aumenta el saldo de deuda y no modifica el saldo de dinero|

## Ejemplo de distribución de cuotas

Una compra de 100 pesos en tres cuotas genera importes de 34, 33 y 33 pesos. Si su fecha es diciembre de 2026, el primer pago propuesto corresponde a enero de 2027; al conservar ese valor, las cuotas corresponden a enero, febrero y marzo de 2027.

Si una tarjeta no tenía deudas, registrar esa compra deja un saldo de deuda de 100 pesos sin afectar el dinero. Pagar la primera cuota genera una salida de 34 pesos en la cuenta pagadora y una disminución de deuda de 34 pesos en la tarjeta, cuyo saldo de deuda queda en 66 pesos.
