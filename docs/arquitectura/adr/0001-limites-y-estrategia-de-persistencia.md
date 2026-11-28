# ADR 0001: límites y estrategia de persistencia

- Estado: Aceptado
- Fecha: 2026-11-27
- Issue: #32

## Contexto

ORBYN necesita persistir estado compartido y estado local sin acoplar sus dominios a motores, drivers ni formatos de almacenamiento. La arquitectura establece PostgreSQL como fuente de verdad compartida y SQLite para capacidades locales. Actualmente existen modelos de dominio, pero el servidor todavía no contiene casos de uso de negocio que lean o escriban esos modelos.

Los estándares del proyecto también establecen que no deben introducirse abstracciones sin un consumidor concreto.

## Decisión

### Autoridad y propiedad de los datos

- PostgreSQL será la autoridad de los datos compartidos entre usuarios, organizaciones, equipos, proyectos, colaboración y estado coordinado.
- SQLite se limitará a datos locales del dispositivo: caché reconstruible, preferencias/estado local y cola de operaciones pendientes cuando exista un flujo offline implementado.
- Una copia en SQLite nunca será autoridad para resolver conflictos de datos compartidos. Las reglas de sincronización y resolución de conflictos requieren una decisión propia antes de implementar escritura offline.
- Los artefactos binarios se almacenarán fuera de las bases relacionales. La base conservará referencias, tamaño y checksums cuando el dominio lo requiera.
- Cada dato tendrá un propietario de dominio explícito. Un dominio no modificará directamente las tablas o estructuras internas de otro dominio.

### Contratos y dependencias

- Los puertos de persistencia pertenecerán a la capa de aplicación que los consume, junto al caso de uso correspondiente; no se definirán repositorios universales en `orbyn-storage`.
- El dominio no dependerá de SQL, SQLx, drivers, conexiones ni tipos específicos de una base de datos.
- `orbyn-storage` alojará adaptadores concretos, configuración y migraciones. Los adaptadores implementarán los puertos del consumidor sin trasladar reglas de negocio a SQL.
- El servidor será el composition root: construirá adaptadores y los inyectará en los casos de uso.
- No se creará un contrato hasta que exista un caso de uso identificable que necesite sustituir o aislar una frontera de persistencia.

### Transacciones y consistencia

- La transacción tendrá el alcance de un caso de uso y cubrirá los cambios que deban confirmarse atómicamente dentro de una misma base.
- Las transacciones no atravesarán PostgreSQL y SQLite ni otros servicios. No se implementarán transacciones distribuidas.
- Las llamadas externas, ejecución de agentes y transferencia de artefactos no se mantendrán dentro de una transacción de base de datos.
- Si un caso de uso necesita coordinar escrituras durables con publicación de eventos, se evaluará un patrón outbox en ese flujo concreto; no se añade infraestructura anticipadamente.

### Identificadores y errores

- Se conservarán los identificadores tipados UUID que ya expone el dominio. La generación ocurre en el límite de creación del agregado y los adaptadores persisten el valor recibido.
- Los errores de dominio y de aplicación conservarán significado propio. Los adaptadores traducirán errores del motor a errores de persistencia estables; no expondrán errores crudos del driver a los dominios.
- Las restricciones de integridad de la base reforzarán invariantes, pero no sustituirán la validación del dominio.

### Migraciones y evolución de esquema

- Las migraciones serán versionadas, revisables y ejecutadas por el adaptador responsable del motor.
- PostgreSQL y SQLite tendrán historiales de migración independientes; sus esquemas no se asumirán idénticos.
- Las migraciones aplicadas en entornos compartidos serán inmutables. Los cambios posteriores se realizarán mediante nuevas migraciones.
- Los cambios incompatibles seguirán una evolución expandir/migrar/contraer cuando deban convivir versiones de la aplicación.
- La versión del esquema se consultará y validará al iniciar el adaptador. La aplicación fallará con un error explícito si el esquema es incompatible; no intentará reparar silenciosamente una base compartida.
- La estrategia concreta de herramienta de migraciones y sus dependencias se decidirá al incorporar el primer adaptador, considerando compatibilidad con Rust estable y el presupuesto cero.

## Alternativas consideradas

### Repositorio genérico en `orbyn-storage`

Rechazado: invierte la propiedad del contrato, fuerza una abstracción común a dominios distintos y puede acoplar la lógica de negocio al módulo de infraestructura.

### SQL directamente desde los dominios

Rechazado: acopla el modelo y sus reglas a un motor, dificulta las pruebas y viola los límites de responsabilidad.

### Añadir PostgreSQL y SQLite ahora

Rechazado: no existe todavía un caso de uso persistente en el servidor. Añadir drivers, configuración y migraciones sin un consumidor aumentaría el coste y la superficie de mantenimiento sin entregar una capacidad funcional.

### Usar SQLite como autoridad compartida

Rechazado: no satisface la necesidad de estado coordinado entre múltiples usuarios y nodos.

## Consecuencias

- Se mantiene la separación entre dominio, aplicación e infraestructura.
- La primera implementación de persistencia deberá incluir un caso de uso real, su puerto mínimo, un adaptador y pruebas proporcionadas al riesgo.
- Las pruebas unitarias de dominio no requerirán bases de datos. Los adaptadores tendrán pruebas de integración y los contratos compartidos solo se añadirán cuando exista comportamiento común verificable.
- Esta decisión no implementa sincronización offline, replicación, almacenamiento de artefactos ni adaptadores de base de datos.

## Próximo paso

Seleccionar un caso de uso de negocio que el servidor vaya a ofrecer primero —por ejemplo, crear y consultar organizaciones o proyectos— y, en esa misma entrega, definir el contrato mínimo que consume. No se debe crear una interfaz de repositorio antes de seleccionar ese flujo y su propietario.
