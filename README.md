# ORBYN

**Plataforma híbrida de orquestación de agentes y colaboración en tiempo real.**

ORBYN permite que equipos de desarrollo coordinen personas, agentes, modelos y herramientas mediante orquestaciones visuales y definiciones como código. Combina una aplicación de escritorio nativa en Rust, un plano de control central y nodos de ejecución locales o remotos.

## Principios

- Rust como lenguaje de implementación de producto.
- Interfaz de escritorio nativa, sin React ni frontend JavaScript.
- Arquitectura híbrida: coordinación central y ejecución distribuida.
- Orquestaciones versionadas, visuales y declarativas.
- Límites explícitos entre dominios, crates y responsabilidades.
- Seguridad, permisos, cuotas y auditoría aplicados en los límites de ejecución.
- Documentación, issues y PRs en español.
- Todo cambio de producto entra mediante Pull Request y controles automatizados.

## Estado

ORBYN se encuentra en etapa de fundación arquitectónica. La estructura y los contratos iniciales no representan todavía un producto funcional ni una garantía de compatibilidad de producción.

## Documentación

- [Arquitectura](docs/arquitectura/README.md)
- [Gobernanza](docs/gobernanza/README.md)
- [Estrategia de ramas](docs/gobernanza/estrategia-de-ramas.md)
- [Estándares de ingeniería](docs/ingenieria/estandares.md)
- [Seguridad](SECURITY.md)
- [Contribución](CONTRIBUTING.md)
- [Código de conducta](CODE_OF_CONDUCT.md)
- [Roadmap](ROADMAP.md)
- [Registro de decisiones](docs/adr/README.md)

## Requisitos de desarrollo

- Rust estable, versión mínima definida en `rust-toolchain.toml`.
- Git.
- Dependencias de sistema requeridas por Iced/wgpu en Linux.

## Comprobaciones locales

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

## Estructura

```text
apps/
  desktop/
  server/
  node/
crates/
  domain/
  events/
  orchestration/
  runtime/
  governance/
  identity/
  collaboration/
  integrations/
  storage/
docs/
  arquitectura/
  adr/
  gobernanza/
  ingenieria/
```

## Licencia

Apache-2.0. Consultar [LICENSE](LICENSE).
