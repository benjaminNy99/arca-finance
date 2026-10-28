# Descripción del proyecto

Nombre: Sistema de gestión de finanzas personales.

Sistema informático local destinado a controlar las finanzas de los usuarios y conocer su dinero entre gastos, ahorros y deudas.

Cada cuenta pertenece a un usuario, tiene un saldo de dinero y un tipo. Las cuentas de deuda y las tarjetas de crédito tienen además un saldo de deuda separado. Su propietario puede eliminarla junto con toda su información asociada. Las deudas y las tarjetas de crédito se representan mediante tipos de cuenta. El acceso requiere autenticación: cada usuario accede a sus propias cuentas y el administrador puede consultar toda la información de cualquier cuenta con acceso de solo lectura.

Los movimientos se distinguen entre movimientos de dinero y movimientos de deuda. Los primeros suman entradas o restan salidas del saldo de dinero; los segundos aumentan o disminuyen el saldo de deuda. Cada transferencia entre cuentas genera dos movimientos de dinero: una salida en origen y una entrada en destino. Los movimientos se pueden buscar por fecha y ordenar para su presentación en listados. Al modificar un movimiento se aplica la diferencia entre su efecto anterior y el nuevo al saldo que afecta.

Al registrar una compra en tarjeta de crédito se indica la cantidad de cuotas y el mes del primer pago, que por defecto es el mes siguiente al de la fecha de compra ingresada. Las cuotas se distribuyen en meses consecutivos y sus importes se calculan en pesos enteros, repartiendo el resto desde las primeras cuotas. La compra registra un movimiento de aumento de deuda por su importe total y conserva el saldo de dinero.

En las cuentas de deuda también se puede registrar una adquisición con su fecha y cuotas pendientes, aumentando el saldo de deuda mediante un movimiento de deuda. Las cuotas son registros independientes de los movimientos y permanecen pendientes hasta su pago. El usuario puede consultar las cuotas pendientes y el total a pagar por mes.

Al pagar una cuota se seleccionan la cuenta pagadora y la fecha de pago, cuyo valor por defecto es el día del pago y puede editarse. Se generan dos movimientos vinculados: una salida de dinero en la cuenta pagadora y una disminución de deuda en la cuenta de deuda o tarjeta de crédito. La cuota deja de estar pendiente.

La auditoría registra la creación y las modificaciones de los movimientos de dinero y de deuda y conserva su responsable, fecha y hora, acción, resultado y cambios cuando corresponda. Los logs registran eventos y comportamientos de la aplicación, excluyendo credenciales y contenido financiero.
