# ORBYN

Plataforma Rust-first para colaboración entre desarrolladores, proyectos y sistemas de IA.

## Estado

ORBYN se encuentra en fase de fundación. La arquitectura, las políticas de desarrollo y la automatización se incorporan progresivamente mediante pull requests.

## Principios

- Rust como lenguaje principal y aplicación desktop nativa, sin Tauri, React ni frontend web embebido.
- Implementaciones propias; otros productos son referencias funcionales, no dependencias de runtime.
- Arquitectura modular con responsabilidades y límites de dominio explícitos.
- Abstracciones justificadas por necesidades reales.
- Seguridad y gobernanza desde el diseño.
- Cambios revisables y trazables mediante pull requests.
- Documentación técnica mantenida junto al código.

## Plataforma inicial

Ubuntu Linux es el único objetivo de desarrollo y CI por ahora.

## Verificación local

    cargo fmt --all -- --check
    cargo check --workspace --all-targets --all-features
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace --all-targets --all-features
    cargo doc --workspace --no-deps --all-features

Consulta CONTRIBUTING.md y docs/architecture/README.md.
