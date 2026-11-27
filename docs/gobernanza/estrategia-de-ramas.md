# Estrategia de ramas

## Ramas permanentes

| Rama | Propósito | Entrada |
|---|---|---|
| `developer` | Integración continua de trabajo revisado | PRs de ramas de tarea |
| `test` | Validación integrada de hitos completos | PR de promoción desde developer |
| `release` | Estabilización de una versión candidata | PR desde test |
| `main` | Código estable aprobado | PR desde release |
| `production` | Estado exacto desplegado | PR desde main |

`main` permanece como rama predeterminada del repositorio y referencia estable. `production` no recibe commits directos ni cambios no trazables a una versión de main.

## Ramas temporales

- `feat/<issue>-<resumen>`
- `fix/<issue>-<resumen>`
- `refactor/<issue>-<resumen>`
- `docs/<issue>-<resumen>`
- `test/<issue>-<resumen>`
- `hotfix/<issue>-<resumen>`

## Flujo

1. Crear issue con alcance y criterios de aceptación.
2. Crear rama de tarea desde developer.
3. Abrir PR hacia developer y completar checks.
4. Integrar cambios de forma continua en developer sin exigir aprobación humana de promoción por cada PR de tarea, siempre que se cumpla la política de revisión configurada.
5. Al completar un hito funcional, abrir una única PR de promoción developer -> test.
6. La promoción a test requiere aprobación humana del propietario y todos los checks.
7. Promover test -> release, release -> main y main -> production mediante PRs independientes, aprobación del propietario y validaciones específicas.
8. Cada promoción registra versión, changelog, artefactos y trazabilidad.

## Reglas no negociables

- Prohibidos pushes directos, force pushes y borrado de ramas permanentes.
- Prohibido fusionar con checks fallidos o pendientes.
- Prohibido omitir revisiones requeridas.
- No se promueven ramas incompletas.
- Los conflictos se resuelven en la rama de origen; no se reescribe una rama protegida.
- Los merges deben conservar trazabilidad de PR e issue.
