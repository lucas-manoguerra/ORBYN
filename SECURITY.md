# Política de seguridad

## Reporte responsable

No publicar vulnerabilidades explotables, credenciales, datos personales ni detalles que permitan comprometer usuarios en issues públicos. Reportar incidentes sensibles al responsable del repositorio mediante un canal privado de GitHub.

## Requisitos de ingeniería

- Denegar por defecto capacidades no concedidas.
- No almacenar secretos en código, workflows, documentación ni logs.
- Referenciar credenciales mediante identificadores opacos y un almacén seguro.
- Aplicar aislamiento del sistema operativo a ejecución de agentes y herramientas.
- Validar identidad, organización, proyecto, permisos y límites en el servidor.
- Usar leases de corta duración para nodos y ejecuciones remotas.
- Auditar acciones sensibles y decisiones de políticas.
- Tratar prompts, resultados de modelos, repositorios y artefactos como entradas no confiables.
- Mantener dependencias auditadas y licencias compatibles.
- No confiar en controles de interfaz como mecanismo de autorización.

## Alcance

Esta política aplica al código, infraestructura, automatizaciones, integraciones y datos gestionados por ORBYN.
