use orbyn_domain::UserId;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityProvider {
    GitHub,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ExternalAccountId(String);

impl ExternalAccountId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        let normalized = value.trim();

        if normalized.is_empty()
            || normalized.chars().count() > 255
            || normalized.chars().any(char::is_control)
        {
            return Err(IdentityError::InvalidExternalAccountId);
        }

        Ok(Self(normalized.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ExternalAccountId {
    type Error = IdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ExternalAccountId> for String {
    fn from(value: ExternalAccountId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExternalIdentity {
    id: Uuid,
    user_id: UserId,
    provider: IdentityProvider,
    external_account_id: ExternalAccountId,
    display_name: Option<String>,
}

impl ExternalIdentity {
    pub fn new(
        user_id: UserId,
        provider: IdentityProvider,
        external_account_id: ExternalAccountId,
        display_name: Option<String>,
    ) -> Result<Self, IdentityError> {
        let display_name = display_name
            .map(|name| name.trim().to_owned())
            .filter(|name| !name.is_empty());

        if display_name
            .as_ref()
            .is_some_and(|name| name.chars().count() > 100 || name.chars().any(char::is_control))
        {
            return Err(IdentityError::InvalidDisplayName);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            provider,
            external_account_id,
            display_name,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn provider(&self) -> IdentityProvider {
        self.provider
    }

    pub fn external_account_id(&self) -> &ExternalAccountId {
        &self.external_account_id
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatedPrincipal {
    user_id: UserId,
}

impl AuthenticatedPrincipal {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }

    pub fn user_id(self) -> UserId {
        self.user_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    InvalidExternalAccountId,
    InvalidDisplayName,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExternalAccountId => {
                formatter.write_str("el identificador externo no es válido")
            }
            Self::InvalidDisplayName => formatter.write_str("el nombre visible no es válido"),
        }
    }
}

impl Error for IdentityError {}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn user_id() -> UserId {
        UserId(Uuid::from_u128(1))
    }

    #[test]
    fn external_account_id_trims_surrounding_whitespace() {
        let result = ExternalAccountId::new("  123456  ");

        assert_eq!(
            result.map(|id| id.as_str().to_owned()),
            Ok("123456".to_owned())
        );
    }

    #[test]
    fn external_account_id_rejects_empty_values() {
        assert_eq!(
            ExternalAccountId::new("  "),
            Err(IdentityError::InvalidExternalAccountId)
        );
    }

    #[test]
    fn external_account_id_rejects_control_characters() {
        assert_eq!(
            ExternalAccountId::new("123\n456"),
            Err(IdentityError::InvalidExternalAccountId)
        );
    }

    #[test]
    fn external_account_id_rejects_values_longer_than_255_characters() {
        assert_eq!(
            ExternalAccountId::new("a".repeat(256)),
            Err(IdentityError::InvalidExternalAccountId)
        );
    }

    #[test]
    fn authenticated_principal_exposes_only_the_internal_user_id() {
        let principal = AuthenticatedPrincipal::new(user_id());

        assert_eq!(principal.user_id(), user_id());
    }

    #[test]
    fn external_identity_keeps_provider_and_stable_account_id() {
        let external_id = ExternalAccountId::new("987654").map(|id| {
            ExternalIdentity::new(
                user_id(),
                IdentityProvider::GitHub,
                id,
                Some("  orbyn-dev  ".to_owned()),
            )
        });

        assert_eq!(
            external_id.map(|result| result.map(|identity| (
                identity.user_id(),
                identity.provider(),
                identity.external_account_id().as_str().to_owned(),
                identity.display_name().map(str::to_owned),
            ))),
            Ok(Ok((
                user_id(),
                IdentityProvider::GitHub,
                "987654".to_owned(),
                Some("orbyn-dev".to_owned()),
            )))
        );
    }

    #[test]
    fn external_identity_rejects_invalid_display_names() {
        let account_id = ExternalAccountId::new("987654");

        assert_eq!(
            account_id.and_then(|id| ExternalIdentity::new(
                user_id(),
                IdentityProvider::GitHub,
                id,
                Some("x".repeat(101)),
            )),
            Err(IdentityError::InvalidDisplayName)
        );
    }
}
