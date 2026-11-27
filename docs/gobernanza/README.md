# Gobernanza del repositorio

## Autoridad

El propietario del repositorio es la autoridad final sobre arquitectura, seguridad, políticas de colaboración y promociones entre ramas protegidas.

## Colaboradores

Los colaboradores pueden trabajar en ramas de tarea, abrir PRs, proponer decisiones y revisar cambios. No pueden aprobar su propio trabajo ni modificar políticas de protección.

## Aprobaciones

Las PRs ordinarias hacia `developer` se validan mediante CI y revisión técnica según CODEOWNERS. Las promociones de hitos hacia `test`, `release`, `main` y `production` requieren aprobación del propietario. El propietario puede aprobar sus propias PRs cuando la política de GitHub lo permita; los colaboradores no pueden sustituir esa aprobación.

## Automatización

La automatización puede validar, etiquetar, solicitar revisiones y habilitar auto-merge cuando corresponda. No puede eludir checks, fabricar aprobaciones ni promover ramas protegidas sin autorización humana.

## Idioma

La documentación, issues, PRs, decisiones y reportes persistentes del proyecto se redactan en español.
