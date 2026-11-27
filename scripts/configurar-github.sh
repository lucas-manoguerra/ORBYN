#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-lucas-manoguerra/ORBYN}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

command -v gh >/dev/null
command -v jq >/dev/null
gh auth status
test "$(gh api "repos/$REPO" --jq '.permissions.admin')" = "true"

gh api --method PATCH "repos/$REPO" \
  -F allow_auto_merge=true \
  -F allow_merge_commit=false \
  -F allow_rebase_merge=false \
  -F allow_squash_merge=true \
  -F allow_update_branch=true \
  -F delete_branch_on_merge=true >/dev/null

while IFS='|' read -r name color description; do
  gh label create "$name" --repo "$REPO" --color "$color" --description "$description" --force
done <<'LABELS'
tipo:funcionalidad|1D76DB|Nueva capacidad de producto
tipo:error|D73A4A|Comportamiento incorrecto
tipo:arquitectura|5319E7|Decisión o cambio arquitectónico
tipo:documentacion|0075CA|Documentación
tipo:dependencias|0366D6|Dependencias
tipo:seguridad|B60205|Seguridad
estado:triage|FBCA04|Pendiente de clasificación
estado:lista|0E8A16|Lista para desarrollo
estado:en-progreso|C2E0C6|En desarrollo
estado:bloqueada|B60205|Bloqueada
prioridad:critica|B60205|Prioridad crítica
prioridad:alta|D93F0B|Prioridad alta
prioridad:media|FBCA04|Prioridad media
prioridad:baja|0E8A16|Prioridad baja
LABELS

while IFS='|' read -r title description; do
  existing="$(gh api --paginate "repos/$REPO/milestones?state=all" --jq '.[].title')"
  if ! printf '%s\n' "$existing" | grep -Fxq "$title"; then
    gh api --method POST "repos/$REPO/milestones" -f "title=$title" -f "description=$description" >/dev/null
  fi
done <<'MILESTONES'
Fase 0 - Fundación y gobernanza|Arquitectura, repositorio, gobernanza y automatización
Fase 1 - Núcleo local|Proyectos, persistencia local y workspace nativo
Fase 2 - Orquestación|Grafo tipado, DSL, versiones y runtime
Fase 3 - Colaboración y gobernanza|Equipos, salas, permisos, cuotas y auditoría
Fase 4 - Ejecución híbrida|Nodos locales/remotos, leases y WebRTC
Fase 5 - Integraciones y extensibilidad|GitHub multiidentidad, modelos y SDK
MILESTONES

MAIN_SHA="$(gh api "repos/$REPO/git/ref/heads/main" --jq '.object.sha')"
for branch in developer test release production; do
  if ! gh api "repos/$REPO/git/ref/heads/$branch" >/dev/null 2>&1; then
    gh api --method POST "repos/$REPO/git/refs" -f "ref=refs/heads/$branch" -f "sha=$MAIN_SHA" >/dev/null
  fi
done

for file in "$ROOT"/.github/rulesets/*.json; do
  name="$(jq -r '.name' "$file")"
  existing="$(gh api --paginate "repos/$REPO/rulesets" --jq '.[].name')"
  if ! printf '%s\n' "$existing" | grep -Fxq "$name"; then
    gh api --method POST "repos/$REPO/rulesets" --input "$file" >/dev/null
  fi
done

printf 'Configuración de ORBYN aplicada en %s\n' "$REPO"
