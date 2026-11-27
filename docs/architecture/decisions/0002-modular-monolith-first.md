# ADR 0002: Monolito modular antes de distribuir servicios

- Estado: Aceptada
- Fecha: 2026-11-27

## Contexto

La distribución prematura aumenta costos de despliegue, observabilidad, consistencia y testing antes de comprender los límites del dominio.

## Decisión

Comenzar con una arquitectura modular y límites explícitos entre dominios y aplicación. La ejecución de agentes será un proceso worker aislado cuando se incorpore. Extraer servicios desplegables solo cuando requisitos medidos justifiquen esa frontera.

## Consecuencias

- Ownership y dirección de dependencias se revisan en código.
- Los contratos internos son explícitos sin exigir un protocolo de red entre cada módulo.
- La separación en servicios sigue siendo posible, pero no es el valor predeterminado.