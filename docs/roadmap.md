# Roadmap

## H1: Establecer la base

Rama: `main`. Preparar la entidad `User` en `arca-core` como base del registro y la autenticación definidos en RF-18 y RF-01 de [requirements.md](requirements.md), [CU-01](use-cases/CU-01.md) y [CU-02](use-cases/CU-02.md).

### Tareas

- [x] Separar el workspace en ejecutable, reglas de negocio y almacenamiento.
- [x] Implementar la validación de contraseñas en `arca-core`.
- [x] Implementar la generación de hashes de contraseñas con Argon2.
- [x] Implementar la verificación de contraseñas contra su hash.
- [ ] Documentar los campos obligatorios del usuario en CU-01.
- [ ] Documentar las reglas de validación de esos campos en CU-01.
- [ ] Implementar la entidad `User` con los campos definidos.
- [ ] Aplicar las validaciones definidas al crear un usuario.
- [ ] Integrar el módulo `password` en la creación de usuarios.
- [ ] Agregar pruebas de creación de usuarios válidos.
- [ ] Agregar pruebas de rechazo de datos de usuario inválidos.
- [ ] Ejecutar `cargo build --workspace --offline`.
- [ ] Ejecutar `cargo test --workspace --offline`.

### Decisiones pendientes

CU-01 aún no define los campos obligatorios ni sus validaciones. Se conserva la decisión previa de usar un nombre de usuario único y una contraseña protegida; falta precisar los demás datos antes de implementar `User`. La comprobación de unicidad mediante almacenamiento corresponde al registro posterior.

### Criterios de cierre

- CU-01 especifica los campos y las validaciones necesarios para construir `User`.
- `arca-core` permite crear un usuario válido y rechaza los datos que incumplen esas reglas.
- `User` conserva el hash de la contraseña y permite verificarla mediante el módulo `password` existente.
- La entidad se mantiene independiente de la terminal y del almacenamiento.
- El build y las pruebas del workspace finalizan correctamente, incluidas las pruebas de usuarios y contraseñas.

El avance marcado corresponde al código presente. Las comprobaciones quedan pendientes para el cierre de H1. Los identificadores del plan anterior se conservan sin marcar en [roadmap-history.md](roadmap-history.md).
