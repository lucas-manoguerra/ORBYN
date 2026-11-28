# Identidad y autenticación con GitHub

## Objetivo

GitHub es el primer proveedor externo de identidad de ORBYN. La identidad externa permite establecer una cuenta interna; no reemplaza el `UserId` propio ni convierte el nombre de usuario de GitHub en una clave.

La autenticación y la autorización son responsabilidades distintas:

- **Autenticación:** demostrar qué cuenta externa controla la persona y resolverla a un `UserId` interno.
- **Autorización en ORBYN:** decidir qué puede hacer ese usuario según su organización, equipo, proyecto y políticas.
- **Acceso a repositorios:** conceder permisos de GitHub por separado y con el alcance mínimo necesario para la función solicitada.

## Identidad estable

- La clave externa es el identificador numérico estable de la cuenta GitHub, no el login/username, correo ni nombre visible.
- La identidad se identifica por el par proveedor + identificador externo.
- El nombre visible es metadato mutable y nunca se utiliza para autorizar.
- Una cuenta externa verificada se asocia a un `UserId` interno.
- No se permite vincular una cuenta existente a otro usuario sin un flujo explícito de prueba de control y resolución de conflictos.

## Flujo futuro de autenticación

1. El cliente inicia el flujo en el servidor ORBYN; no recibe ni conoce el client secret.
2. El servidor crea un valor `state` impredecible, de un solo uso y con vencimiento corto. Se valida exactamente en el callback para impedir CSRF.
3. Se usa PKCE cuando el flujo y tipo de aplicación de GitHub lo soporten; el verifier se conserva únicamente en el servidor o cliente confidencial correspondiente.
4. El redirect URI se configura explícitamente y se compara de forma exacta. No se aceptan redirects arbitrarios.
5. El servidor intercambia el código por el token con GitHub. El código OAuth no se registra ni se persiste como dato de negocio.
6. El servidor consulta la API autenticada de GitHub y obtiene el identificador estable de la cuenta. No confía en un ID, username o perfil enviado por el navegador.
7. Se resuelve o crea la asociación con el usuario interno según una política de vinculación explícita.
8. ORBYN emite su propia sesión, con expiración, revocación y protección contra CSRF cuando se usen cookies. El cliente nunca construye un principal enviando un `user_id`.
9. El callback elimina o invalida el estado temporal incluso cuando el flujo falla.

## Scopes y acceso a repositorios

- El inicio de sesión solicita solo los permisos de perfil estrictamente necesarios.
- No se solicita acceso general a repositorios como requisito para iniciar sesión.
- La conexión de repositorios es una autorización separada y explícita.
- Para operaciones de repositorio se evaluará GitHub Apps y permisos granulares por instalación/repositorio, evitando scopes amplios cuando sea posible.
- Los scopes concedidos se tratan como metadatos de una autorización, no como roles de ORBYN.

## Secretos, tokens y persistencia

- Client secrets, access tokens, refresh tokens, códigos OAuth y PKCE verifiers no pertenecen a los modelos de dominio ni a structs serializables de identidad.
- Nunca se escriben en logs, trazas, errores, URLs persistidas ni respuestas al cliente.
- Los secretos de configuración se inyectan en el proceso del servidor mediante configuración segura; no se versionan en Git.
- Los tokens que deban conservarse se guardarán mediante un adaptador dedicado, cifrados en reposo y con acceso mínimo. La clave de cifrado debe mantenerse fuera de la base de datos.
- La revocación/desconexión elimina o invalida las credenciales asociadas sin borrar el historial de autoría interno.
- La persistencia de asociaciones y credenciales se implementará cuando el caso de uso requiera un adaptador durable, siguiendo el ADR 0001.

## Frontera de código

- `orbyn-identity` define el principal interno y los tipos de identidad externa; no realiza llamadas HTTP ni contiene secretos.
- Un adaptador de proveedor verifica la respuesta de GitHub y produce una identidad externa confiable.
- La capa de aplicación resuelve el vínculo con el usuario interno y aplica las políticas de asociación.
- El servidor compone los adaptadores, valida el callback y establece la sesión.
- Los casos de uso reciben un principal autenticado tipado; no reciben un ID de usuario elegido por el cliente.

## Estado

Este documento define límites y requisitos de seguridad. No implementa OAuth, sesiones HTTP, almacenamiento de tokens ni acceso a repositorios. Esas capacidades tendrán entregas separadas con pruebas de seguridad y configuración explícita.
