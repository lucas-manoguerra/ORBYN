# Contribuir a ORBYN

## Reglas de ingeniería

1. Las reglas de dominio no dependen de frameworks, bases de datos, transportes ni proveedores.
2. Cada comportamiento pertenece al dominio que lo gobierna.
3. Las dependencias deben ser explícitas y dirigirse hacia las políticas del dominio.
4. Preferir módulos cohesivos, APIs públicas pequeñas y detalles privados.
5. Añadir pruebas para invariantes, errores, autorización y regresiones.
6. Registrar decisiones arquitectónicas que afecten límites, dependencias, persistencia o seguridad.
7. Nunca subir secretos, credenciales, datos personales ni artefactos locales.
8. Unsafe Rust queda prohibido por defecto.

## Verificación local

    cargo fmt --all -- --check
    cargo check --workspace --all-targets --all-features
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace --all-targets --all-features
    cargo doc --workspace --no-deps --all-features

Los PR deben ser enfocados, explicar intención y alcance, incluir pruebas y documentación, y mantener CI verde.