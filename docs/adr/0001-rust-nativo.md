# ADR 0001: Aplicación nativa en Rust

- Estado: Aceptado
- Fecha: 2026-11-27

## Contexto

ORBYN requiere una aplicación de escritorio con límites de responsabilidad claros y una base tecnológica coherente.

## Decisión

La aplicación de escritorio, el servidor, los nodos y los componentes de producto se implementan en Rust. No se utiliza React ni un frontend JavaScript.

## Consecuencias

Se prioriza una UI nativa Rust. Deben validarse accesibilidad, rendimiento, editor de grafos y empaquetado Linux desde las primeras iteraciones.
