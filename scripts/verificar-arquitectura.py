import json
import subprocess
import sys
from pathlib import Path

root = Path(__file__).resolve().parents[1]
policy_path = root / "architecture-boundaries.json"
policy = json.loads(policy_path.read_text(encoding="utf-8"))
metadata = json.loads(
    subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=root,
        text=True,
    )
)
packages = {
    package["name"]: package
    for package in metadata["packages"]
    if package["id"] in metadata["workspace_members"]
}
errors = []

if set(policy) != set(packages):
    missing = sorted(set(packages) - set(policy))
    stale = sorted(set(policy) - set(packages))
    if missing:
        errors.append(f"Paquetes sin política de dependencias: {', '.join(missing)}")
    if stale:
        errors.append(f"Políticas para paquetes inexistentes: {', '.join(stale)}")

for package_name, package in packages.items():
    allowed = set(policy.get(package_name, []))
    for dependency in package["dependencies"]:
        dependency_name = dependency["name"]
        if dependency_name in packages and dependency_name not in allowed:
            errors.append(
                f"{package_name} depende de {dependency_name} sin autorización arquitectónica"
            )
    unknown = allowed - set(packages)
    if unknown:
        errors.append(
            f"{package_name} autoriza paquetes inexistentes: {', '.join(sorted(unknown))}"
        )

if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)

print(f"Arquitectura validada: {len(packages)} paquetes y límites explícitos")
