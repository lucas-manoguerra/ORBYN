# Modelo de orquestación

## Representación

Una orquestación se compone de nodos tipados y aristas con contratos de entrada y salida. Los nodos iniciales previstos son trigger, agent, model, tool, condition, parallel, join, human approval, sub-workflow y output.

## Versionado

Una versión publicada es inmutable. Cada ejecución registra el identificador y la versión exacta. Las suborquestaciones fijan una versión explícita; no se resuelve una referencia mutable durante una ejecución.

## Validación

Antes de publicar se verifican identificadores únicos, referencias existentes, tipos de puertos, nodos inalcanzables, ciclos no soportados, capacidades requeridas y límites de recursos.

## DSL

La definición como código y el editor visual comparten el mismo modelo canónico. Ninguna de las dos interfaces define un runtime independiente.
