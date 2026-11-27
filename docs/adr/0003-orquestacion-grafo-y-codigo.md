# ADR 0003: Orquestación visual y declarativa

- Estado: Aceptado
- Fecha: 2026-11-27

## Contexto

Los equipos necesitan editar workflows visualmente y mantenerlos versionados, revisables y reutilizables como código.

## Decisión

El grafo tipado es el modelo canónico. El editor visual y la definición declarativa compilan al mismo modelo y validación.

## Consecuencias

Se requiere versionado inmutable, validación de tipos y contratos de nodos, y ejecución fijada a una versión.
