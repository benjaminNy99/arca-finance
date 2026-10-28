# Módulos

1. **Usuarios y acceso:** Identificación, autenticación y acceso a cuentas propias. Consulta de toda la información de las cuentas por el administrador con acceso de solo lectura.
2. **Cuentas financieras:** Representación de cuentas y sus tipos, con saldo de dinero y, en deudas y tarjetas de crédito, saldo de deuda separado; eliminación de cuentas propias y toda su información asociada.
3. **Movimientos:** Registro de movimientos de dinero o deuda, transferencias de dinero entre cuentas, búsqueda por fecha y orden de presentación en listados. Actualización del saldo afectado por cada movimiento y ajuste por diferencia de efecto al modificarlo.
4. **Deudas, compras y cuotas:** Registro de adquisiciones de deuda y compras en tarjeta de crédito con movimientos de aumento de deuda y cuotas pendientes. Distribución de compras en cuotas mensuales en pesos enteros, consulta de pagos futuros y pago de cuotas mediante movimientos vinculados de dinero y deuda.
5. **Auditoría:** Registro de la creación y las modificaciones de movimientos de dinero o deuda, con responsable, fecha y hora, cuenta, acción, resultado y valores anteriores y posteriores cuando existan cambios.
6. **Logs del sistema:** Registro de eventos de ejecución, autenticación, accesos denegados, operaciones y errores, con fecha y hora, severidad, evento, resultado y datos de correlación cuando correspondan. Exclusión de credenciales y contenido financiero.
