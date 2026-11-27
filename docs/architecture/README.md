# Arquitectura ORBYN

## Intención

Organizar el software por capacidades de negocio y límites explícitos, no por productos externos que inspiraron funcionalidades.

## Principios

- Domain-Driven Design para definir límites, lenguaje e invariantes.
- Clean/Hexagonal Architecture para mantener las reglas de dominio independientes de infraestructura y delivery.
- Los casos de uso de aplicación coordinan operaciones; las entidades mantienen invariantes.
- Los puertos pertenecen a la política que los necesita; los adaptadores viven en los bordes.
- Las abstracciones requieren una necesidad demostrable: variación, repetición o testabilidad.
- Empezar con un monolito modular. Separar servicios solo por necesidades medidas de seguridad, aislamiento, escala u operación.
- La ejecución de agentes tendrá aislamiento de proceso por su capacidad de ejecutar herramientas y modificar archivos.

## Mapa inicial de dominios

- Identity & Access: usuarios internos, identidades externas, sesiones, roles y autorización.
- Organization & Collaboration: organizaciones, workspaces, membresías, grupos y canales.
- Messaging & Realtime: conversaciones, mensajes, hilos, presencia y entrega de eventos.
- Media: salas, participantes, audio, video y pantalla compartida.
- Projects & Source Control: proyectos, repositorios, ramas, cambios y revisiones.
- Model Management: catálogo de modelos, configuración de proveedores, capacidades y referencias a secretos.
- Agent Platform: agentes, herramientas, sesiones, contexto, memoria y ejecuciones.
- Orchestration: workflows, grafos de tareas, delegación, scheduling, reintentos y aprobaciones.
- Developer Network: perfiles, publicaciones, seguidores, descubrimiento y publicación.

Este mapa no obliga a crear inmediatamente un crate o microservicio por dominio. Extraer crates cuando sus límites y ownership sean estables y útiles.

## Dirección de dependencias

Presentación e infraestructura dependen de contratos de aplicación. Los casos de uso dependen de políticas de dominio y puertos. El dominio no importa UI, drivers SQL, clientes HTTP, SDK de GitHub ni SDK de proveedores de modelos.

## Plataforma

Ubuntu Linux es el único objetivo soportado por CI inicialmente. La compatibilidad multiplataforma se añadirá con cobertura CI explícita.