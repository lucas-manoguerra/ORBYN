use async_trait::async_trait;
use orbyn_domain::UserId;
use orbyn_identity::AuthenticatedPrincipal;
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionFamilyId(Uuid);

impl SessionFamilyId {
    pub fn as_uuid(self) -> Uuid { self.0 }
}

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
    pub fn id(&self) -> SessionId {
        self.id
    }
    pub fn family_id(&self) -> SessionFamilyId {
        self.family_id
    }
    pub fn user_id(&self) -> UserId {
        self.user_id
    }
    pub fn digests(&self) -> &CredentialDigests {
        &self.digests
    }
    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
    pub fn access_expires_at_unix_ms(&self) -> i64 {
        self.access_expires_at_unix_ms
    }
    pub fn refresh_expires_at_unix_ms(&self) -> i64 {
        self.refresh_expires_at_unix_ms
    }
    pub fn absolute_expires_at_unix_ms(&self) -> i64 {
        self.absolute_expires_at_unix_ms
    }
    pub fn revoked_at_unix_ms(&self) -> Option<i64> {
        self.revoked_at_unix_ms
    }

    pub fn apply_rotation(
        &mut self,
        replacement: SessionCredentialRotation,
        now_unix_ms: i64,
    ) -> bool {
        if self.revoked_at_unix_ms.is_some()
            || now_unix_ms >= self.refresh_expires_at_unix_ms
            || now_unix_ms >= self.absolute_expires_at_unix_ms
            || replacement.access_expires_at_unix_ms <= now_unix_ms
            || replacement.refresh_expires_at_unix_ms <= now_unix_ms
            || replacement.access_expires_at_unix_ms > self.absolute_expires_at_unix_ms
        {
            return false;
        }
        let refresh_expiry = replacement
            .refresh_expires_at_unix_ms
            .min(self.absolute_expires_at_unix_ms);
        if refresh_expiry < replacement.access_expires_at_unix_ms {
            return false;
        }
        self.digests = replacement.digests;
        self.access_expires_at_unix_ms = replacement.access_expires_at_unix_ms;
        self.refresh_expires_at_unix_ms = refresh_expiry;
        true
    }

    pub fn revoke_at(&mut self, now_unix_ms: i64) {
        self.revoked_at_unix_ms = Some(now_unix_ms);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCredentialRotation {
    digests: CredentialDigests,
    access_expires_at_unix_ms: i64,
    refresh_expires_at_unix_ms: i64,
}

impl SessionCredentialRotation {
    pub fn new(
        digests: CredentialDigests,
        access_expires_at_unix_ms: i64,
        refresh_expires_at_unix_ms: i64,
    ) -> Result<Self, SessionError> {
        if access_expires_at_unix_ms < 0
            || refresh_expires_at_unix_ms < access_expires_at_unix_ms
        {
            return Err(SessionError::InvalidSessionTransition);
        }
        Ok(Self {
            digests,
            access_expires_at_unix_ms,
            refresh_expires_at_unix_ms,
        })
    }

    pub fn digests(&self) -> &CredentialDigests {
        &self.digests
    }
    pub fn access_expires_at_unix_ms(&self) -> i64 {
        self.access_expires_at_unix_ms
    }
    pub fn refresh_expires_at_unix_ms(&self) -> i64 {
        self.refresh_expires_at_unix_ms
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

pub trait CredentialProvider: Send + Sync {
    fn generate(&self) -> Result<String, CredentialProviderError>;
    fn digest(&self, credential: &str) -> Result<Vec<u8>, CredentialProviderError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("No se pudo procesar la credencial")]
pub struct CredentialProviderError;

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
pub enum RepositoryRotationResult {
    Rotated(SessionRecord),
    Invalid,
    ReuseDetected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionRotationOutcome {
    Rotated(IssuedSession),
    Invalid,
    ReuseDetected,
}

#[derive(Clone, PartialEq, Eq)]
pub struct IssuedSession {
    pub session_id: SessionId,
    pub access_credential: SecretCredential,
    pub refresh_credential: SecretCredential,
    pub access_expires_at_unix_ms: i64,
    pub refresh_expires_at_unix_ms: i64,
    pub absolute_expires_at_unix_ms: i64,
}

impl fmt::Debug for IssuedSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuedSession")
            .field("session_id", &self.session_id)
            .field("access_credential", &"[REDACTED]")
            .field("refresh_credential", &"[REDACTED]")
            .field("access_expires_at_unix_ms", &self.access_expires_at_unix_ms)
            .field(
                "refresh_expires_at_unix_ms",
                &self.refresh_expires_at_unix_ms,
            )
            .field(
                "absolute_expires_at_unix_ms",
                &self.absolute_expires_at_unix_ms,
            )
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SessionError {
    #[error("La política de sesión no es válida")]
    InvalidPolicy,
    #[error("La marca de tiempo no es válida")]
    InvalidTimestamp,
    #[error("El verificador de credencial no es válido")]
    InvalidCredentialDigest,
    #[error("La transición de sesión no es válida")]
    InvalidSessionTransition,
    #[error("No se pudo procesar la credencial")]
    CredentialProvider,
    #[error("No se pudo completar la operación de sesión")]
    Repository(SessionRepositoryError),
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn insert(&self, session: SessionRecord) -> Result<(), SessionRepositoryError>;

    async fn authenticate_access(
        &self,
        access_digest: &[u8],
        now_unix_ms: i64,
    ) -> Result<Option<UserId>, SessionRepositoryError>;

    async fn rotate_refresh(
        &self,
        presented_digest: &[u8],
        replacement: SessionCredentialRotation,
        now_unix_ms: i64,
    ) -> Result<RepositoryRotationResult, SessionRepositoryError>;

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
        if now < 0 {
            return Err(SessionError::InvalidTimestamp);
        }

        let access_expiry = now
            .checked_add(self.policy.access_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;
        let refresh_expiry = now
            .checked_add(self.policy.refresh_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;
        let absolute_expiry = now
            .checked_add(self.policy.absolute_lifetime_ms)
            .ok_or(SessionError::InvalidTimestamp)?;

        let access = self.new_credential()?;
        let refresh = self.new_credential()?;
        let session = SessionRecord {
            id: SessionId(Uuid::new_v4()),
            family_id: SessionFamilyId(Uuid::new_v4()),
            user_id,
            digests: self.digest_pair(&access, &refresh)?,
            created_at_unix_ms: now,
            access_expires_at_unix_ms: access_expiry,
            refresh_expires_at_unix_ms: refresh_expiry,
            absolute_expires_at_unix_ms: absolute_expiry,
            revoked_at_unix_ms: None,
        };

        self.repository
            .insert(session.clone())
            .await
            .map_err(SessionError::Repository)?;
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
        if access_credential.is_empty() {
            return Ok(None);
        }
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }
        let digest = self
            .credentials
            .digest(access_credential)
            .map_err(|_| SessionError::CredentialProvider)?;
        if digest.is_empty() {
            return Err(SessionError::InvalidCredentialDigest);
        }
        let user_id = self
            .repository
            .authenticate_access(&digest, now)
            .await
            .map_err(SessionError::Repository)?;
        Ok(user_id.map(AuthenticatedPrincipal::new))
    }

    pub async fn rotate(
        &self,
        refresh_credential: &str,
    ) -> Result<SessionRotationOutcome, SessionError> {
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }

        if refresh_credential.is_empty() {
            return Ok(SessionRotationOutcome::Invalid);
        }
        let presented_digest = self
            .credentials
            .digest(refresh_credential)
            .map_err(|_| SessionError::CredentialProvider)?;
        if presented_digest.is_empty() { return Err(SessionError::InvalidCredentialDigest); }

        let access = self.new_credential()?;
        let refresh = self.new_credential()?;
        let replacement = SessionCredentialRotation::new(
            self.digest_pair(&access, &refresh)?,
            now.checked_add(self.policy.access_lifetime_ms)
                .ok_or(SessionError::InvalidTimestamp)?,
            now.checked_add(self.policy.refresh_lifetime_ms)
                .ok_or(SessionError::InvalidTimestamp)?,
        )?;

        match self
            .repository
            .rotate_refresh(&presented_digest, replacement, now)
            .await
            .map_err(SessionError::Repository)?
        {
            RepositoryRotationResult::Rotated(record) => {
                Ok(SessionRotationOutcome::Rotated(IssuedSession {
                    session_id: record.id,
                    access_credential: access,
                    refresh_credential: refresh,
                    access_expires_at_unix_ms: record.access_expires_at_unix_ms,
                    refresh_expires_at_unix_ms: record.refresh_expires_at_unix_ms,
                    absolute_expires_at_unix_ms: record.absolute_expires_at_unix_ms,
                }))
            }
            RepositoryRotationResult::Invalid => Ok(SessionRotationOutcome::Invalid),
            RepositoryRotationResult::ReuseDetected => Ok(SessionRotationOutcome::ReuseDetected),
        }
    }

    pub async fn revoke(&self, session_id: SessionId) -> Result<bool, SessionError> {
        let now = self.clock.now_unix_ms();
        if now < 0 { return Err(SessionError::InvalidTimestamp); }
        self.repository
            .revoke(session_id, now)
            .await
            .map_err(SessionError::Repository)
    }

    fn new_credential(&self) -> Result<SecretCredential, SessionError> {
        self.credentials
            .generate()
            .map(SecretCredential::new)
            .map_err(|_| SessionError::CredentialProvider)
    }

    fn digest_pair(
        &self,
        access: &SecretCredential,
        refresh: &SecretCredential,
    ) -> Result<CredentialDigests, SessionError> {
        let access = self
            .credentials
            .digest(access.expose_once())
            .map_err(|_| SessionError::CredentialProvider)?;
        let refresh = self
            .credentials
            .digest(refresh.expose_once())
            .map_err(|_| SessionError::CredentialProvider)?;
        CredentialDigests::new(access, refresh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{
            Arc, Mutex,
            atomic::{AtomicI64, AtomicUsize, Ordering},
        },
    };

    #[derive(Clone, Default)]
    struct TestClock(Arc<AtomicI64>);

    impl TestClock {
        fn set(&self, value: i64) {
            self.0.store(value, Ordering::SeqCst);
        }
    }

    impl SessionClock for TestClock {
        fn now_unix_ms(&self) -> i64 {
            self.0.load(Ordering::SeqCst)
        }
    }

    #[derive(Clone, Default)]
    struct TestCredentials(Arc<AtomicUsize>);

    impl CredentialProvider for TestCredentials {
        fn generate(&self) -> Result<String, CredentialProviderError> {
            Ok(format!(
                "test-secret-{}",
                self.0.fetch_add(1, Ordering::SeqCst)
            ))
        }

        fn digest(&self, credential: &str) -> Result<Vec<u8>, CredentialProviderError> {
            Ok(credential.as_bytes().to_vec())
        }
    }

    struct StoredSession {
        record: SessionRecord,
        consumed_refresh_digests: Vec<Vec<u8>>,
    }

    #[derive(Default)]
    struct InMemorySessionRepository {
        sessions: Mutex<HashMap<SessionId, StoredSession>>,
    }

    #[async_trait]
    impl SessionRepository for InMemorySessionRepository {
        async fn insert(&self, session: SessionRecord) -> Result<(), SessionRepositoryError> {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|_| SessionRepositoryError::Unavailable)?;
            if sessions.contains_key(&session.id) {
                return Err(SessionRepositoryError::Conflict);
            }
            sessions.insert(
                session.id,
                StoredSession {
                    record: session,
                    consumed_refresh_digests: Vec::new(),
                },
            );
            Ok(())
        }

        async fn authenticate_access(
            &self,
            access_digest: &[u8],
            now_unix_ms: i64,
        ) -> Result<Option<UserId>, SessionRepositoryError> {
            let sessions = self
                .sessions
                .lock()
                .map_err(|_| SessionRepositoryError::Unavailable)?;
            Ok(sessions
                .values()
                .find(|stored| {
                    stored.record.digests.access() == access_digest
                        && stored.record.revoked_at_unix_ms.is_none()
                        && now_unix_ms < stored.record.access_expires_at_unix_ms
                        && now_unix_ms < stored.record.absolute_expires_at_unix_ms
                })
                .map(|stored| stored.record.user_id))
        }

        async fn rotate_refresh(
            &self,
            presented_digest: &[u8],
            replacement: SessionCredentialRotation,
            now_unix_ms: i64,
        ) -> Result<RepositoryRotationResult, SessionRepositoryError> {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|_| SessionRepositoryError::Unavailable)?;
            let current_id = sessions
                .iter()
                .find(|(_, stored)| stored.record.digests.refresh() == presented_digest)
                .map(|(id, _)| *id);

            if let Some(id) = current_id {
                let Some(stored) = sessions.get_mut(&id) else {
                    return Ok(RepositoryRotationResult::Invalid);
                };
                if stored.record.revoked_at_unix_ms.is_some()
                    || now_unix_ms >= stored.record.refresh_expires_at_unix_ms
                    || now_unix_ms >= stored.record.absolute_expires_at_unix_ms
                {
                    return Ok(RepositoryRotationResult::Invalid);
                }
                let mut updated = stored.record.clone();
                if !updated.apply_rotation(replacement, now_unix_ms) {
                    return Ok(RepositoryRotationResult::Invalid);
                }
                stored
                    .consumed_refresh_digests
                    .push(presented_digest.to_vec());
                stored.record = updated.clone();
                return Ok(RepositoryRotationResult::Rotated(updated));
            }

            let reused_family = sessions
                .values()
                .find(|stored| {
                    stored
                        .consumed_refresh_digests
                        .iter()
                        .any(|digest| digest == presented_digest)
                })
                .map(|stored| stored.record.family_id);

            if let Some(family_id) = reused_family {
                for stored in sessions
                    .values_mut()
                    .filter(|stored| stored.record.family_id == family_id)
                {
                    stored.record.revoke_at(now_unix_ms);
                }
                return Ok(RepositoryRotationResult::ReuseDetected);
            }

            Ok(RepositoryRotationResult::Invalid)
        }

        async fn revoke(
            &self,
            session_id: SessionId,
            now_unix_ms: i64,
        ) -> Result<bool, SessionRepositoryError> {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|_| SessionRepositoryError::Unavailable)?;
            let Some(family_id) = sessions
                .get(&session_id)
                .map(|stored| stored.record.family_id)
            else {
                return Ok(false);
            };
            for stored in sessions
                .values_mut()
                .filter(|stored| stored.record.family_id == family_id)
            {
                stored.record.revoke_at(now_unix_ms);
            }
            Ok(true)
        }
    }

    type TestApplication =
        SessionApplication<InMemorySessionRepository, TestClock, TestCredentials>;

    fn application() -> Option<(TestApplication, TestClock)> {
        let clock = TestClock::default();
        clock.set(1_000);
        let result = SessionApplication::new(
            InMemorySessionRepository::default(),
            clock.clone(),
            TestCredentials::default(),
            SessionPolicy {
                access_lifetime_ms: 100,
                refresh_lifetime_ms: 500,
                absolute_lifetime_ms: 1_000,
            },
        );
        result.ok().map(|app| (app, clock))
    }

    fn user_id() -> UserId {
        UserId(Uuid::from_u128(42))
    }

    async fn create_session(app: &TestApplication) -> Option<IssuedSession> {
        let result = app.create(user_id()).await;
        assert!(result.is_ok());
        result.ok()
    }

    #[tokio::test]
    async fn creates_session_and_authenticates_principal() {
        let result = application();
        assert!(result.is_some());
        let Some((app, _)) = result else {
            return;
        };
        let Some(issued) = create_session(&app).await else {
            return;
        };
        let principal = app
            .authenticate(issued.access_credential.expose_once())
            .await;
        assert!(matches!(principal, Ok(Some(principal)) if principal.user_id() == user_id()));
    }

    #[tokio::test]
    async fn rejects_expired_access_credentials() {
        let result = application();
        assert!(result.is_some());
        let Some((app, clock)) = result else {
            return;
        };
        let Some(issued) = create_session(&app).await else { return; };
        clock.set(1_100);
        assert!(matches!(
            app.authenticate(issued.access_credential.expose_once())
                .await,
            Ok(None)
        ));
    }

    #[tokio::test]
    async fn rotates_credentials_and_invalidates_the_previous_access_token() {
        let result = application();
        assert!(result.is_some());
        let Some((app, _)) = result else { return; };
        let Some(issued) = create_session(&app).await else { return; };
        let rotated = app.rotate(issued.refresh_credential.expose_once()).await;
        assert!(matches!(rotated, Ok(SessionRotationOutcome::Rotated(_))));
        let Ok(SessionRotationOutcome::Rotated(next)) = rotated else {
            return;
        };
        assert!(matches!(
            app.authenticate(issued.access_credential.expose_once())
                .await,
            Ok(None)
        ));
        assert!(matches!(
            app.authenticate(next.access_credential.expose_once()).await,
            Ok(Some(_))
        ));
    }

    #[tokio::test]
    async fn refresh_reuse_revokes_the_entire_session_family() {
        let result = application();
        assert!(result.is_some());
        let Some((app, _)) = result else { return; };
        let Some(issued) = create_session(&app).await else { return; };
        let rotated = app.rotate(issued.refresh_credential.expose_once()).await;
        assert!(matches!(rotated, Ok(SessionRotationOutcome::Rotated(_))));
        let Ok(SessionRotationOutcome::Rotated(next)) = rotated else { return; };
        assert!(matches!(
            app.rotate(issued.refresh_credential.expose_once()).await,
            Ok(SessionRotationOutcome::ReuseDetected)
        ));
        assert!(matches!(
            app.authenticate(next.access_credential.expose_once()).await,
            Ok(None)
        ));
    }

    #[tokio::test]
    async fn concurrent_refresh_attempts_detect_reuse_and_revoke_the_family() {
        let result = application();
        assert!(result.is_some());
        let Some((app, _)) = result else { return; };
        let Some(issued) = create_session(&app).await else { return; };
        let credential = issued.refresh_credential.expose_once().to_owned();
        let (first, second) = tokio::join!(app.rotate(&credential), app.rotate(&credential));
        let outcomes = [first, second];
        assert!(
            outcomes
                .iter()
                .any(|outcome| matches!(outcome, Ok(SessionRotationOutcome::Rotated(_))))
        );
        assert!(
            outcomes
                .iter()
                .any(|outcome| matches!(outcome, Ok(SessionRotationOutcome::ReuseDetected)))
        );
    }

    #[tokio::test]
    async fn revoke_invalidates_access_and_refresh_credentials() {
        let result = application();
        assert!(result.is_some());
        let Some((app, _)) = result else { return; };
        let Some(issued) = create_session(&app).await else { return; };
        assert!(matches!(app.revoke(issued.session_id).await, Ok(true)));
        assert!(matches!(
            app.authenticate(issued.access_credential.expose_once())
                .await,
            Ok(None)
        ));
        assert!(matches!(
            app.rotate(issued.refresh_credential.expose_once()).await,
            Ok(SessionRotationOutcome::Invalid)
        ));
    }

    #[tokio::test]
    async fn rejects_invalid_session_policy() {
        let result = SessionApplication::new(
            InMemorySessionRepository::default(),
            TestClock::default(),
            TestCredentials::default(),
            SessionPolicy {
                access_lifetime_ms: 500,
                refresh_lifetime_ms: 100,
                absolute_lifetime_ms: 1_000,
            },
        );
        assert!(matches!(result, Err(SessionError::InvalidPolicy)));
    }
}
