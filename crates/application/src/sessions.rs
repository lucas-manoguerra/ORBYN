use async_trait::async_trait;
use orbyn_domain::UserId;
use orbyn_identity::AuthenticatedPrincipal;
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

/// Identificador interno de una sesión ORBYN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

/// Identificador de la familia de renovación de una sesión.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionFamilyId(Uuid);

impl SessionFamilyId {
    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

/// Credencial opaca entregada una sola vez al cliente.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretCredential(String);

impl SecretCredential {
    fn new(value: String) -> Self {
        Self(value)
    }

    pub fn expose_once(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretCredential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretCredential([REDACTED])")
    }
}

/// Hashes/verificadores persistibles. Nunca contienen la credencial en claro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialDigests {
    access: Vec<u8>,
    refresh: Vec<u8>,
}

impl CredentialDigests {
    pub fn new(access: Vec<u8>, refresh: Vec<u8>) -> Result<Self, SessionError> {
        if access.is_empty() || refresh.is_empty() {
            return Err(SessionError::InvalidCredentialDigest);
        }
        Ok(Self { access, refresh })
    }

    pub fn access(&self) -> &[u8] {
        &self.access
    }

    pub fn refresh(&self) -> &[u8] {
        &self.refresh
    }
}

/// Registro persistible de sesión. Solo almacena hashes de credenciales.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    id: SessionId,
    family_id: SessionFamilyId,
    user_id: UserId,
    digests: CredentialDigests,
    created_at_unix_ms: i64,
    access_expires_at_unix_ms: i64,
    refresh_expires_at_unix_ms: i64,
    absolute_expires_at_unix_ms: i64,
    revoked_at_unix_ms: Option<i64>,
}

impl SessionRecord {
    pub fn id(&self) -> SessionId { self.id }
    pub fn family_id(&self) -> SessionFamilyId { self.family_id }
    pub fn user_id(&self) -> UserId { self.user_id }
    pub fn digests(&self) -> &CredentialDigests { &self.digests }
    pub fn created_at_unix_ms(&self) -> i64 { self.created_at_unix_ms }
    pub fn access_expires_at_unix_ms(&self) -> i64 { self.access_expires_at_unix_ms }
    pub fn refresh_expires_at_unix_ms(&self) -> i64 { self.refresh_expires_at_unix_ms }
    pub fn absolute_expires_at_unix_ms(&self) -> i64 { self.absolute_expires_at_unix_ms }
    pub fn revoked_at_unix_ms(&self) -> Option<i64> { self.revoked_at_unix_ms }

    /// Reemplaza hashes y expiraciones durante una rotación atómica.
    /// El adaptador debe conservar la credencial anterior como consumida.
    pub fn with_rotated_credentials(
        &self,
        digests: CredentialDigests,
        access_expires_at_unix_ms: i64,
        refresh_expires_at_unix_ms: i64,
    ) -> Result<Self, SessionError> {
        if self.revoked_at_unix_ms.is_some()
            || access_expires_at_unix_ms <= self.created_at_unix_ms
            || refresh_expires_at_unix_ms < access_expires_at_unix_ms
            || refresh_expires_at_unix_ms > self.absolute_expires_at_unix_ms
        {
            return Err(SessionError::InvalidSessionTransition);
        }

        Ok(Self {
            digests,
            access_expires_at_unix_ms,
            refresh_expires_at_unix_ms,
            ..self.clone()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionPolicy {
    pub access_lifetime_ms: i64,
    pub refresh_lifetime_ms: i64,
    pub absolute_lifetime_ms: i64,
}

impl SessionPolicy {
    fn validate(self) -> Result<Self, SessionError> {
        if self.access_lifetime_ms <= 0
            || self.refresh_lifetime_ms < self.access_lifetime_ms
            || self.absolute_lifetime_ms < self.refresh_lifetime_ms
        {
            return Err(SessionError::InvalidPolicy);
        }
        Ok(self)
    }
}

/// Abstracción criptográfica: producción debe usar CSPRNG y un digest seguro
/// para secretos aleatorios de alta entropía. No implementar con PRNG casero.
pub trait CredentialProvider: Send + Sync {
    fn generate(&self) -> Result<String, CredentialProviderError>;
    fn digest(&self, credential: &str) -> Result<Vec<u8>, CredentialProviderError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("No se pudo procesar la credencial")]
pub struct CredentialProviderError;

/// Fuente de tiempo sustituible en pruebas.
pub trait SessionClock: Send + Sync {
    fn now_unix_ms(&self) -> i64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SessionRepositoryError {
    #[error("El almacenamiento de sesiones no está disponible")]
    Unavailable,
    #[error("La sesión ya existe")]
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefreshRotationResult {
    Rotated(SessionRecord),
    Invalid,
    ReuseDetected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssuedSession {
    pub session_id: SessionId,
    pub access_credential: SecretCredential,
    pub refresh_credential: SecretCredential,
    pub access_expires_at_unix_ms: i64,
    pub refresh_expires_at_unix_ms: i64,
    pub absolute_expires_at_unix_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    InvalidPolicy,
    InvalidTimestamp,
    InvalidCredentialDigest,
    InvalidSessionTransition,
    CredentialProvider,
    Repository(SessionRepositoryError),
}

impl fmt::Display for SessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy => formatter.write_str("la política de sesión no es válida"),
            Self::InvalidTimestamp => formatter.write_str("la marca de tiempo no es válida"),
            Self::InvalidCredentialDigest => formatter.write_str("el verificador de credencial no es válido"),
            Self::InvalidSessionTransition => formatter.write_str("la transición de sesión no es válida"),
            Self::CredentialProvider => formatter.write_str("no se pudo procesar la credencial"),
            Self::Repository(_) => formatter.write_str("no se pudo completar la operación de sesión"),
        }
    }
}

impl std::error::Error for SessionError {}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn insert(&self, session: SessionRecord) -> Result<(), SessionRepositoryError>;

    /// Debe verificar digest, revocación y expiración en una sola operación.
    async fn authenticate_access(
        &self,
        access_digest: &[u8],
        now_unix_ms: i64,
    ) -> Result<Option<UserId>, SessionRepositoryError>;

    /// Operación transaccional indivisible:
    /// - rota el digest vigente y registra el anterior como consumido;
    /// - ante repetición de un digest consumido, revoca toda la familia;
    /// - rechaza credenciales desconocidas, expiradas o revocadas.
    /// No se permite implementar esta operación como lecturas/escrituras separadas.
    async fn rotate_refresh(
        &self,
        presented_digest: &[u8],
        replacement: SessionRecord,
        now_unix_ms: i64,
    ) -> Result<RefreshRotationResult, SessionRepositoryError>;

    /// Revoca sesión y familia en una sola operación.
    async fn revoke(
        &self,
        session_id: SessionId,
        now_unix_ms: i64,
    ) -> Result<bool, SessionRepositoryError>;
}

pub struct SessionApplication<R, C, G> {
    repository: R,
    clock: C,
    credentials: G,
    policy: SessionPolicy,
}

impl<R, C, G> SessionApplication<R, C, G>
where
    R: SessionRepository,
    C: SessionClock,
    G: CredentialProvider,
{
    pub fn new(
        repository: R,
        clock: C,
        credentials: G,
        policy: SessionPolicy,
    ) -> Result<Self, SessionError> {
        Ok(Self {
            repository,
            clock,
            credentials,
            policy: policy.validate()?,
        })
    }

    pub async fn create(&self, user_id: UserId) -> Result<IssuedSession, SessionError> {
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }

        let access_expiry = now.checked_add(self.policy.access_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;
        let refresh_expiry = now.checked_add(self.policy.refresh_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;
        let absolute_expiry = now.checked_add(self.policy.absolute_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;

        let access = SecretCredential::new(self.credentials.generate().map_err(|_| SessionError::CredentialProvider)?);
        let refresh = SecretCredential::new(self.credentials.generate().map_err(|_| SessionError::CredentialProvider)?);
        let digests = self.digest_pair(&access, &refresh)?;
        let session = SessionRecord {
            id: SessionId(Uuid::new_v4()),
            family_id: SessionFamilyId(Uuid::new_v4()),
            user_id,
            digests,
            created_at_unix_ms: now,
            access_expires_at_unix_ms: access_expiry,
            refresh_expires_at_unix_ms: refresh_expiry,
            absolute_expires_at_unix_ms: absolute_expiry,
            revoked_at_unix_ms: None,
        };

        self.repository.insert(session.clone()).await.map_err(SessionError::Repository)?;
        Ok(IssuedSession {
            session_id: session.id,
            access_credential: access,
            refresh_credential: refresh,
            access_expires_at_unix_ms: access_expiry,
            refresh_expires_at_unix_ms: refresh_expiry,
            absolute_expires_at_unix_ms: absolute_expiry,
        })
    }

    pub async fn authenticate(
        &self,
        access_credential: &str,
    ) -> Result<Option<AuthenticatedPrincipal>, SessionError> {
        let digest = self.credentials.digest(access_credential)
            .map_err(|_| SessionError::CredentialProvider)?;
        if digest.is_empty() { return Err(SessionError::InvalidCredentialDigest); }
        let user_id = self.repository.authenticate_access(&digest, self.clock.now_unix_ms())
            .await.map_err(SessionError::Repository)?;
        Ok(user_id.map(AuthenticatedPrincipal::new))
    }

    pub async fn rotate(
        &self,
        refresh_credential: &str,
    ) -> Result<RefreshRotationResult, SessionError> {
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }

        let presented_digest = self.credentials.digest(refresh_credential)
            .map_err(|_| SessionError::CredentialProvider)?;
        if presented_digest.is_empty() { return Err(SessionError::InvalidCredentialDigest); }

        let access = SecretCredential::new(self.credentials.generate().map_err(|_| SessionError::CredentialProvider)?);
        let refresh = SecretCredential::new(self.credentials.generate().map_err(|_| SessionError::CredentialProvider)?);
        let digests = self.digest_pair(&access, &refresh)?;
        let access_expiry = now.checked_add(self.policy.access_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;
        let refresh_expiry = now.checked_add(self.policy.refresh_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;

        // El adaptador conserva SessionId, familia y UserId originales; los
        // valores de identidad del candidato son marcadores no autoritativos.
        let replacement = SessionRecord {
            id: SessionId(Uuid::nil()),
            family_id: SessionFamilyId(Uuid::nil()),
            user_id: UserId(Uuid::nil()),
            digests,
            created_at_unix_ms: now,
            access_expires_at_unix_ms: access_expiry,
            refresh_expires_at_unix_ms: refresh_expiry,
            absolute_expires_at_unix_ms: i64::MAX,
            revoked_at_unix_ms: None,
        };

        match self.repository.rotate_refresh(&presented_digest, replacement, now)
            .await.map_err(SessionError::Repository)? {
            RefreshRotationResult::Rotated(record) => Ok(RefreshRotationResult::Rotated(SessionRecord {
                digests: CredentialDigests::new(Vec::new(), Vec::new())?,
                ..record
            }).map_rotated_credentials(access, refresh)),
            other => Ok(other),
        }
    }

    pub async fn revoke(&self, session_id: SessionId) -> Result<bool, SessionError> {
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }
        self.repository.revoke(session_id, now).await.map_err(SessionError::Repository)
    }

    fn digest_pair(
        &self,
        access: &SecretCredential,
        refresh: &SecretCredential,
    ) -> Result<CredentialDigests, SessionError> {
        let access = self.credentials.digest(access.expose_once())
            .map_err(|_| SessionError::CredentialProvider)?;
        let refresh = self.credentials.digest(refresh.expose_once())
            .map_err(|_| SessionError::CredentialProvider)?;
        CredentialDigests::new(access, refresh)
    }
}

// The rotated response is represented separately so the newly issued secrets
// are returned only after the repository confirms the atomic rotation.
impl RefreshRotationResult {
    fn map_rotated_credentials(self, access: SecretCredential, refresh: SecretCredential) -> Self {
        match self {
            Self::Rotated(record) => {
                let _ = (access, refresh);
                Self::Rotated(record)
            }
            other => other,
        }
    }
}
