# Configuración administrativa de GitHub

## Requisitos

- GitHub CLI instalado y autenticado.
- Permiso de administrador sobre el repositorio.
- Permiso de escritura de administración para crear rulesets.
- La PR de fundación debe estar integrada en main antes de ejecutar el script.

## Aplicación

Desde la raíz del repositorio:

```bash
bash scripts/configurar-github.sh
```

El script es idempotente para etiquetas, ramas y rulesets por nombre. Crea las ramas permanentes desde el HEAD de main. No elimina ni modifica ramas existentes.

## Ramas creadas

- developer
- test
- release
- production

main permanece como rama predeterminada.

## Reglas

Cada rama protegida exige PR, historial lineal, prohibición de borrado y force-push, y los checks de CI versionados. Las promociones hacia test, release, main y production requieren revisión de CODEOWNERS.

El rol administrador del repositorio dispone de bypass limitado a los requisitos de revisión en PR para permitir que el propietario apruebe su propia promoción. Los checks de calidad y las reglas de integridad están en rulesets separados, sin bypass, por lo que esta excepción no permite saltar CI ni force-push.

## Verificación posterior

```bash
gh api repos/lucas-manoguerra/ORBYN/rulesets
gh api repos/lucas-manoguerra/ORBYN/rules/branches/developer
gh api repos/lucas-manoguerra/ORBYN/rules/branches/test
gh api repos/lucas-manoguerra/ORBYN/branches
```

La API de creación de rulesets requiere permiso de administración de repositorio. Consultar la [documentación oficial de GitHub](https://docs.github.com/es/rest/repos/rules).
