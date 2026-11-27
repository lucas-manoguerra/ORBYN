# Límites de responsabilidad

## Regla general

Cada crate posee un dominio, un conjunto de contratos públicos y una política explícita de dependencias. Ningún crate puede convertirse en un contenedor de utilidades generales ni asumir la responsabilidad de otro dominio.

| Dominio | Responsabilidad | No debe hacer |
|---|---|---|
| domain | Tipos e invariantes compartidas | Acceso a red, disco o proveedores |
| events | Contratos de eventos y envelope | Orquestar procesos o persistir |
| orchestration | Definición, validación y compilación de grafos | Ejecutar agentes o gestionar credenciales |
| runtime | Ejecutar agentes, modelos y herramientas | Decidir membresías o administrar organizaciones |
| governance | Evaluar capacidades, políticas y cuotas | Ejecutar herramientas |
| identity | Identidades y referencias de credenciales | Resolver secretos dentro de workflows |
| collaboration | Salas, presencia y mensajes | Ejecutar tareas de agentes |
| integrations | Adaptadores de proveedores externos | Definir políticas globales |
| storage | Adaptadores de persistencia y migraciones | Contener reglas de negocio de otros dominios |

## Reglas de dependencias

- Los contratos compartidos se ubican en el dominio más pequeño que los posee.
- Las implementaciones dependen de contratos; los dominios no dependen de infraestructura.
- Las dependencias cruzadas deben estar justificadas en arquitectura y revisadas.
- No se permiten ciclos de dependencias entre crates.
- La comunicación entre dominios se realiza mediante contratos, puertos o eventos versionados.
- La capa de aplicación compone dominios; no absorbe sus responsabilidades.

## Cambios de límites

Toda modificación de límites, propiedad de datos o dirección de dependencias requiere ADR. Los cambios deben incluir plan de migración y pruebas de arquitectura.
