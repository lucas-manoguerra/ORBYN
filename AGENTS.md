# Instrucciones para agentes de desarrollo

## Autoridad

Estas reglas son obligatorias para cualquier agente automatizado o asistente que modifique este repositorio. Ante conflicto, prevalecen las políticas de seguridad, gobernanza y arquitectura del repositorio.

## Límites

- Trabajar únicamente dentro del alcance del issue asignado.
- No cambiar contratos de otros dominios sin coordinación y aprobación explícita.
- No realizar cambios en ramas principales ni modificar reglas de protección.
- No aprobar, fusionar ni cerrar PRs por cuenta propia.
- No crear issues, ramas o PRs fuera del flujo definido sin autorización.
- No ampliar permisos, secretos, capacidades de ejecución ni límites de colaboradores.
- No introducir dependencias de UI web ni React. El producto se implementa en Rust.
- No añadir comentarios en código fuente.

## Calidad

- Leer la documentación del módulo antes de modificarlo.
- Mantener dependencias dirigidas hacia contratos estables.
- Preferir abstracciones cuando separen responsabilidades, permitan sustitución o reduzcan acoplamiento real.
- Evitar abstracciones especulativas, capas vacías y duplicación.
- Añadir pruebas unitarias, de integración o de contrato según corresponda.
- Actualizar README, documentación, ADRs, issues y PRs afectados.
- Ejecutar todos los controles locales disponibles antes de solicitar revisión.
- No afirmar que una prueba pasó si no fue ejecutada y observada.

## Comunicación

Toda documentación y comunicación persistente en GitHub se redacta en español. Resumir decisiones, riesgos, archivos modificados y verificaciones en la PR.
