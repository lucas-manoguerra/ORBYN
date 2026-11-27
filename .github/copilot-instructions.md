# Instrucciones de revisión para ORBYN

## Contexto del producto

ORBYN es una plataforma de colaboración y orquestación de agentes. La implementación del producto es Rust-first y la aplicación de escritorio es nativa. No introducir Tauri, React ni un frontend JavaScript como sustitutos de la arquitectura elegida.

## Criterios de revisión

- Priorizar corrección, seguridad, mantenibilidad y límites claros entre dominios.
- Respetar la separación entre aplicaciones y crates; evitar dependencias invertidas o acoplamientos innecesarios.
- Mantener la lógica de dominio independiente de persistencia, transporte, interfaz y proveedores externos.
- Validar invariantes en los límites de entrada y representar errores recuperables con tipos explícitos; evitar panics en caminos normales.
- Revisar permisos, autenticación, autorización, manejo de secretos y exposición de datos en cada frontera de confianza.
- Evitar operaciones de red, filesystem o procesos sin límites, validación y tratamiento de errores.
- No agregar dependencias, abstracciones o capas sin una necesidad concreta.
- Mantener compatibilidad con el toolchain estable definido por el repositorio.
- No ampliar la lista de licencias permitidas ni ignorar avisos de seguridad sin una decisión explícita y documentada.
- No introducir comentarios en código Rust: las decisiones y explicaciones deben quedar en documentación externa, según la política CI del proyecto.
- Verificar que los cambios incluyan pruebas para invariantes, errores y regresiones relevantes.
- No afirmar que tests, compilación o validaciones locales se ejecutaron si no existe evidencia.

## Cómo reportar hallazgos

- Informar primero problemas concretos que puedan causar bugs, vulnerabilidades, pérdida de datos o romper contratos.
- Para cada hallazgo indicar archivo/línea, escenario de fallo e impacto.
- Distinguir errores verificables de sugerencias de estilo.
- No solicitar cambios especulativos ni bloquear por preferencias personales.
- No considerar esta revisión como reemplazo de CI ni de la responsabilidad del mantenedor.
