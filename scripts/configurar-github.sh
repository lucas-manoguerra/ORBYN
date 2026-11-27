#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-lucas-manoguerra/ORBYN}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

command -v gh >/dev/null
gh auth status
test "$(gh api "repos/$REPO" --jq '.permissions.admin')" = "true"

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

printf 'Etiquetas, ramas y rulesets configurados en %s\n' "$REPO"
