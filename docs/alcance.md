# Alcance

Sistema local de control financiero centrado en cuentas, movimientos, deudas y cuotas de compras con tarjeta de crédito, con auditoría y acceso según el usuario.

## Alcance funcional

- **Usuarios y acceso:** Identificación y autenticación, acceso a cuentas propias y consulta de toda la información de cualquier cuenta por el administrador, con acceso de solo lectura.
- **Cuentas:** Cuentas propias con saldo de dinero y tipo. Las cuentas de deuda y las tarjetas de crédito tienen además un saldo de deuda separado. El propietario puede eliminar una cuenta y toda su información asociada.
- **Movimientos:** Registro de movimientos de dinero o de deuda, búsqueda por fecha y orden de presentación configurable. Los movimientos de dinero suman entradas o restan salidas en el saldo de dinero; los de deuda aumentan o disminuyen el saldo de deuda. Las transferencias generan dos movimientos de dinero. Al modificar un movimiento se ajusta el saldo afectado por la diferencia de efecto.
- **Deudas:** Registro de la fecha de adquisición y las cuotas pendientes, con un movimiento de deuda que aumenta el saldo de deuda sin modificar el saldo de dinero.
- **Compras y cuotas:** Registro de compras en tarjeta de crédito indicando cantidad de cuotas y mes del primer pago. Por defecto se propone el mes siguiente al de la fecha de compra. Las cuotas corresponden a meses consecutivos y sus importes se calculan en pesos enteros, repartiendo el resto desde las primeras cuotas. La compra genera un movimiento de deuda por el total y aumenta el saldo de deuda.
- **Pagos futuros:** Consulta de cuotas pendientes e importes por mes. El pago permite elegir la cuenta pagadora y la fecha, que por defecto es la del día del pago; genera dos movimientos vinculados que reducen el saldo de dinero de la cuenta pagadora y el saldo de deuda correspondiente, y retira la cuota de los pendientes.
- **Auditoría:** Registro de la creación y modificación de movimientos de dinero y deuda, con responsable, fecha y hora, cuenta, acción, resultado y valores anteriores y posteriores cuando existan cambios.
- **Logs:** Registro del inicio y cierre de la aplicación, autenticación, accesos denegados, operaciones y errores, con información de diagnóstico y sin credenciales ni contenido financiero.

## Funcionalidades no contempladas en los requisitos actuales

- Integración automática con bancos.
- Importación y exportación de archivos.
- Presupuestos y metas de ahorro.
- Informes, estadísticas o gráficos adicionales a la consulta de cuotas e importes pendientes por mes.
- Notificaciones y alertas.
- Manejo de múltiples monedas.
