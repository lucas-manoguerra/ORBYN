# ADR 0002: Arquitectura híbrida

- Estado: Aceptado
- Fecha: 2026-11-27

## Contexto

La colaboración multiusuario requiere coordinación compartida, mientras que la ejecución de agentes debe permanecer cerca del workspace y bajo control del usuario.

## Decisión

Separar plano de control central, plano de colaboración y nodos de ejecución locales o remotos.

## Consecuencias

Se requieren enrolamiento de nodos, leases temporales, autorización por capacidad, sincronización de eventos y comportamiento offline explícito.
