# Estándares de ingeniería

## Código

- Rust estable y edición 2024.
- Código claro, pequeño y orientado a responsabilidades.
- Cero comentarios en código fuente.
- Documentación pública de APIs mediante documentación Rust cuando sea necesaria; no utilizar comentarios explicativos inline.
- Evitar estado global mutable y acoplamiento oculto.
- Manejar errores con tipos explícitos en límites de dominio.
- No usar panics para errores recuperables.
- No ignorar errores ni usar unwrap/expect en rutas de producto sin justificación excepcional revisada.
- Evitar dependencias nuevas cuando la biblioteca estándar o una dependencia existente resuelvan el caso con calidad.
- No introducir unsafe sin ADR y revisión especializada.

## Abstracciones

Introducir una abstracción cuando exista una frontera de responsabilidad, una necesidad real de sustitución, una política transversal o una dependencia externa que deba aislarse. No introducir traits, factories, managers o capas sin consumidor concreto.

## Pruebas

Cada cambio incluye pruebas proporcionadas al riesgo: unitarias para invariantes, integración para adaptadores y persistencia, contratos para APIs/eventos y pruebas end-to-end para flujos críticos.

## Documentación

Actualizar la documentación afectada en el mismo PR. Los cambios de arquitectura requieren ADR. El README refleja el estado real del producto, no promesas futuras.
