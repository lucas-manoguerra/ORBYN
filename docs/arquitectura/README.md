# Arquitectura de ORBYN

## Objetivo

ORBYN es una plataforma híbrida para coordinar equipos humanos, agentes, modelos y herramientas. La arquitectura separa responsabilidades de control, ejecución y colaboración.

## Planos

### Plano de control

Es autoridad sobre identidad, organizaciones, equipos, proyectos, políticas, workflows publicados, registro de nodos, programación y auditoría.

### Plano de ejecución

Ejecuta tareas en nodos locales o remotos autorizados. Gestiona leases, aislamiento, capacidades, cancelación, reintentos, checkpoints y artefactos.

### Plano de colaboración

Gestiona salas, presencia, mensajes, actividad compartida y señalización WebRTC. El tráfico de audio y video no debe pasar por el servidor de control salvo necesidad explícita.

## Topología híbrida

El servidor coordina y conserva el estado compartido. Los nodos ejecutan trabajo cerca de los repositorios y recursos. Desktop puede actuar también como nodo local cuando el usuario lo autoriza.

## Dominios

- Identidad y organizaciones.
- Proyectos y espacios de trabajo.
- Orquestaciones y versiones.
- Runtime de agentes y modelos.
- Gobernanza y políticas.
- Colaboración y comunicación.
- Integraciones y credenciales.
- Persistencia y eventos.

Cada dominio expone contratos explícitos. No se permite acceder a detalles internos de otro dominio.

## Persistencia

PostgreSQL será la fuente de verdad compartida. SQLite se utilizará para caché, cola offline y estado local. Los artefactos se almacenarán fuera de la base relacional y se referenciarán mediante identificadores y checksums.

## Orquestaciones

El grafo tipado es la representación ejecutable canónica. La definición como código se compila y valida hacia el mismo modelo. Las versiones publicadas son inmutables y cada ejecución fija una versión concreta.

## Seguridad

Las autorizaciones se evalúan en el servidor y en las fronteras de ejecución. Los nodos reciben capacidades acotadas y leases temporales. Los secretos se resuelven desde un vault y nunca forman parte de las definiciones de workflow.
