# Sesiones ORBYN: contratos y garantías

## Responsabilidades

El módulo `orbyn-application::sessions` implementa los casos de uso de creación, autenticación, rotación y revocación. No conoce HTTP, OAuth, GitHub, SQL ni el almacenamiento local del cliente.

- `SessionRecord` es el estado persistible y solo contiene hashes/verificadores de credenciales.
- `IssuedSession` contiene los secretos opacos que se entregan al cliente únicamente en la respuesta de creación o rotación. Su representación `Debug` redacta ambos secretos.
- `CredentialProvider` es un puerto obligatorio para generación criptográfica y digest de credenciales.
- `SessionClock` permite controlar el tiempo en pruebas.
- `SessionRepository` define las operaciones persistentes que los adaptadores deben garantizar.

## Requisitos criptográficos del adaptador

La implementación de producción de `CredentialProvider` debe:

1. Generar credenciales con un generador criptográficamente seguro del sistema operativo (CSPRNG), con al menos 256 bits de entropía.
2. Codificar el resultado como un token opaco apto para transporte, sin incluir identidad, permisos ni datos de sesión.
3. Calcular un digest criptográfico con separación de dominio; dado que el secreto es aleatorio y de alta entropía, no se necesita un hash lento de contraseñas.
4. Comparar/verificar mediante el digest y nunca almacenar ni registrar la credencial original.
5. Fallar de forma cerrada si el generador o digest no está disponible.

No existe aún un adaptador criptográfico de producción. Los proveedores deterministas del módulo de pruebas no son aptos para producción.

## Garantías transaccionales del repositorio

### Crear

`insert` persiste la sesión completa en una transacción. Solo se almacenan `SessionId`, familia, `UserId`, digests y marcas de tiempo. Un conflicto de ID no debe sobrescribir una sesión existente.

### Autenticar

`authenticate_access` compara el digest y verifica revocación, expiración de acceso y duración absoluta en una única operación consistente. La aplicación construye `AuthenticatedPrincipal` únicamente a partir del `UserId` recuperado del repositorio; nunca acepta un ID enviado por el cliente.

### Rotar

`rotate_refresh` debe ejecutar una transacción indivisible:

1. Buscar el digest de renovación vigente.
2. Comprobar que la sesión y la familia no estén revocadas y que no hayan expirado.
3. Registrar el digest presentado como consumido.
4. Sustituir los digests de acceso y renovación por los nuevos.
5. Limitar la expiración de renovación a la expiración absoluta original.

Si el digest presentado ya fue consumido, la operación revoca toda la familia dentro de la misma transacción y devuelve `ReuseDetected`. Un digest desconocido, expirado o revocado devuelve `Invalid`. Dos rotaciones concurrentes no pueden aceptar el mismo digest: una puede ganar y la otra debe detectar la repetición y revocar la familia conforme a ADR 0002.

Los digests consumidos deben conservarse durante la vida máxima de la familia para que la detección de reutilización siga siendo posible. El adaptador durable deberá definir índices únicos y bloqueo/aislamiento adecuados; no puede simular esta operación mediante lecturas y escrituras independientes.

### Revocar

`revoke` revoca sesión y familia en una transacción. Tras confirmar la operación, las autenticaciones y renovaciones posteriores deben fallar.

## Política temporal

- La duración de acceso es menor que la de renovación.
- La duración de renovación no supera la duración absoluta.
- Una rotación nunca amplía la duración absoluta.
- Los tiempos se expresan en milisegundos Unix y deben ser no negativos.
- El servidor debe definir valores de producción explícitos; los valores de pruebas no constituyen una política operativa.

## Estado de implementación

Esta entrega define los casos de uso, puertos y un adaptador en memoria exclusivo de pruebas. No implementa todavía PostgreSQL, migraciones, endpoints, OAuth, almacén de credenciales de Desktop ni un proveedor criptográfico de producción. Esos componentes se incorporarán en entregas separadas y deberán cumplir estas garantías.

Relacionado: ADR 0001, ADR 0002 e issue #44.
