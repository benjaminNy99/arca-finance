# Arca finance

*Sistema de gestión de finanzas personales de código abierto, basado en cuentas, transacciones, deudas y estimaciones financieras.*

Desarrollado en Rust con una interfaz de terminal interactiva.

## Crates del proyecto

| Crate | Responsabilidad |
|---|---|
| `arca-finance` | Ejecutable y terminal interactiva. Conecta los componentes y configura los logs. |
| [arca-core](arca-core/) | Tipos y reglas de negocio de usuarios, cuentas, movimientos y auditoría. |
| [arca-storage](arca-storage/) | Persistencia local y guardado consistente de las operaciones. |

El ejecutable utiliza ambos crates. `arca-storage` usará los tipos de `arca-core`, que se mantiene independiente de la terminal y el almacenamiento.

## Documentación

1. [Descripción](docs/descripcion.md)
2. [Alcance](docs/alcance.md)
3. [Requerimientos](docs/requerimientos.md)
4. [Módulos](docs/modulos.md)
5. [Casos de uso](docs/casos-de-uso.md)
6. [Reglas de negocio](docs/reglas-de-negocio.md)
7. [Roadmap](docs/roadmap.md)
