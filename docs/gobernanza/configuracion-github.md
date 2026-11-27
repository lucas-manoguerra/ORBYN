# Configuración administrativa de GitHub

## Requisitos

- GitHub CLI y jq instalados.
- Sesión autenticada con permisos administrativos sobre el repositorio.
- Permiso de escritura de administración para crear rulesets.
- La PR de fundación integrada en main.

## Aplicación

Desde la raíz del repositorio:

```bash
bash scripts/configurar-github.sh
```

El script configura merge automático, squash como único método, eliminación de ramas temporales tras merge, etiquetas, milestones, ramas permanentes y rulesets. Es idempotente por nombre para etiquetas, milestones, ramas y rulesets. No elimina ramas ni reglas existentes.

## Ramas

- main: rama predeterminada y código estable.
- developer: integración continua de cambios revisados.
- test: validación de hitos completos.
- release: estabilización de versiones candidatas.
- production: estado desplegado.

Las ramas permanentes se crean desde el HEAD de main si todavía no existen.

## Reglas y aprobación del propietario

Los rulesets de integridad y CI son independientes de los rulesets de revisión humana. El propietario del repositorio tiene un bypass limitado a las reglas de revisión en PR, mediante su identidad GitHub. Los checks de CI, la prohibición de borrado, el historial lineal y la prohibición de force-push permanecen en rulesets separados sin bypass.

Los colaboradores no tienen bypass: las promociones a test, release, main y production requieren revisión de CODEOWNERS. El propietario puede autorizar su propia promoción mediante el bypass auditable de la revisión, pero no saltar los checks obligatorios.

## Verificación

```bash
gh api repos/lucas-manoguerra/ORBYN/rulesets
gh api repos/lucas-manoguerra/ORBYN/rules/branches/developer
gh api repos/lucas-manoguerra/ORBYN/rules/branches/test
gh api repos/lucas-manoguerra/ORBYN/branches
gh api repos/lucas-manoguerra/ORBYN/actions/permissions
```

La creación de rulesets requiere permiso de administración del repositorio. Consultar la [documentación oficial de la API de reglas de GitHub](https://docs.github.com/es/rest/repos/rules).
