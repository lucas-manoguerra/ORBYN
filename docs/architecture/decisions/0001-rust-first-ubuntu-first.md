# ADR 0001: Rust-first y Ubuntu-first

- Estado: Aceptada
- Fecha: 2026-11-27

## Contexto

ORBYN prioriza calidad de código, modularidad, límites claros y crecimiento orgánico. El entorno inicial de desarrollo y CI es Ubuntu Linux.

## Decisión

Rust será el lenguaje principal y Cargo Workspaces la estructura de los proyectos Rust. CI se ejecutará inicialmente solo en Ubuntu. La aplicación desktop no utilizará Tauri, React ni frontend web embebido.

## Consecuencias

- Toolchain y verificaciones estandarizadas sobre Rust stable.
- No se declara soporte para plataformas no probadas.
- La tecnología concreta de UI desktop se decidirá mediante un ADR posterior.