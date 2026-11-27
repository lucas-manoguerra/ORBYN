# Guía de contribución

## Idioma

Issues, Pull Requests, documentación, ADRs, mensajes de revisión y comunicación técnica del repositorio se redactan en español. Los nombres técnicos, APIs, identificadores y términos estándar pueden conservarse en inglés cuando mejoren la precisión.

## Antes de trabajar

1. Buscar si ya existe un issue relacionado.
2. Crear o solicitar un issue que describa problema, alcance, criterios de aceptación y límites de responsabilidad.
3. Confirmar que el cambio pertenece al módulo asignado.
4. Crear una rama de trabajo desde `developer` con el formato `feat/<issue>-<resumen>`, `fix/<issue>-<resumen>`, `docs/<issue>-<resumen>` o `refactor/<issue>-<resumen>`.

## Reglas obligatorias

- No realizar pushes directos a ramas principales.
- No mezclar cambios de dominios independientes en una misma PR.
- No introducir dependencias entre crates que violen la arquitectura documentada.
- No agregar comentarios al código fuente. Explicar decisiones y contratos mediante nombres claros, documentación Markdown, ADRs y documentación pública de APIs.
- No incorporar secretos, tokens, credenciales ni datos personales.
- No desactivar, debilitar ni eludir controles de CI.
- Toda funcionalidad debe incluir pruebas proporcionadas al riesgo y actualizar la documentación afectada.
- Los cambios incompatibles requieren ADR y estrategia de migración.
- Una PR debe ser pequeña, revisable y vinculada a un issue.

## Pull Requests

La plantilla debe completarse íntegramente. La PR debe indicar alcance, módulos afectados, decisiones, pruebas ejecutadas, impacto de seguridad, documentación actualizada y plan de reversión cuando corresponda.

La integración ordinaria de cambios completos se realiza en `developer`. Las promociones entre ramas principales se realizan mediante PRs de hito. Las promociones hacia `test`, `release`, `main` y `production` requieren aprobación humana del responsable del repositorio, además de todos los checks aplicables.

## Calidad local

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

## Revisión

La revisión evalúa corrección, límites de responsabilidad, seguridad, mantenibilidad, pruebas, compatibilidad, observabilidad y documentación. La aprobación no reemplaza los checks automatizados.
