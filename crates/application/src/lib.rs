use async_trait::async_trait;
use orbyn_domain::{DomainError, Organization, OrganizationId, UserId};
use orbyn_identity::{ExternalAccountId, ExternalIdentity, IdentityError, IdentityProvider};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum OrganizationRepositoryError {
    #[error("La organización ya existe")]
    Conflict,
    #[error("El almacenamiento no está disponible")]
    Unavailable,
}

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn insert(&self, organization: Organization) -> Result<(), OrganizationRepositoryError>;

    async fn find_by_id(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Option<Organization>, OrganizationRepositoryError>;
}

#[derive(Debug, Error)]
pub enum OrganizationApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] OrganizationRepositoryError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOrganization {
    pub name: String,
    pub owner_user_id: UserId,
    pub created_at_unix_ms: i64,
}

pub struct OrganizationApplication<R> {
    repository: R,
}

impl<R> OrganizationApplication<R>
where
    R: OrganizationRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        command: CreateOrganization,
    ) -> Result<Organization, OrganizationApplicationError> {
        let organization = Organization::new(
            command.name,
            command.owner_user_id,
            command.created_at_unix_ms,
        )?;

        self.repository.insert(organization.clone()).await?;
        Ok(organization)
    }

    pub async fn find(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Option<Organization>, OrganizationApplicationError> {
        self.repository
            .find_by_id(organization_id)
            .await
            .map_err(OrganizationApplicationError::from)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ExternalIdentityRepositoryError {
    #[error("La identidad externa ya está vinculada")]
    Conflict,
    #[error("El almacenamiento de identidades no está disponible")]
    Unavailable,
}

#[async_trait]
pub trait ExternalIdentityRepository: Send + Sync {
    async fn insert(
        &self,
        identity: ExternalIdentity,
    ) -> Result<(), ExternalIdentityRepositoryError>;

    async fn find_by_external_account(
        &self,
        provider: IdentityProvider,
        external_account_id: &ExternalAccountId,
    ) -> Result<Option<ExternalIdentity>, ExternalIdentityRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkExternalIdentity {
    pub user_id: UserId,
    pub provider: IdentityProvider,
    pub external_account_id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum ExternalIdentityApplicationError {
    #[error(transparent)]
    Identity(#[from] IdentityError),
    #[error(transparent)]
    Repository(#[from] ExternalIdentityRepositoryError),
}

pub struct ExternalIdentityApplication<R> {
    repository: R,
}

impl<R> ExternalIdentityApplication<R>
where
    R: ExternalIdentityRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn link(
        &self,
        command: LinkExternalIdentity,
    ) -> Result<ExternalIdentity, ExternalIdentityApplicationError> {
        let external_account_id = ExternalAccountId::new(command.external_account_id)?;
        let identity = ExternalIdentity::new(
            command.user_id,
            command.provider,
            external_account_id,
            command.display_name,
        )?;

        self.repository.insert(identity.clone()).await?;
        Ok(identity)
    }

    pub async fn find(
        &self,
        provider: IdentityProvider,
        external_account_id: &str,
    ) -> Result<Option<ExternalIdentity>, ExternalIdentityApplicationError> {
        let external_account_id = ExternalAccountId::new(external_account_id)?;
        self.repository
            .find_by_external_account(provider, &external_account_id)
            .await
            .map_err(ExternalIdentityApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Mutex};
    use uuid::Uuid;

    #[derive(Default)]
    struct InMemoryOrganizationRepository {
        organizations: Mutex<HashMap<OrganizationId, Organization>>,
    }

    #[async_trait]
    impl OrganizationRepository for InMemoryOrganizationRepository {
        async fn insert(
            &self,
            organization: Organization,
        ) -> Result<(), OrganizationRepositoryError> {
            let mut organizations = self
                .organizations
                .lock()
                .map_err(|_| OrganizationRepositoryError::Unavailable)?;

            if organizations.contains_key(&organization.id()) {
                return Err(OrganizationRepositoryError::Conflict);
            }

            organizations.insert(organization.id(), organization);
            Ok(())
        }

        async fn find_by_id(
            &self,
            organization_id: OrganizationId,
        ) -> Result<Option<Organization>, OrganizationRepositoryError> {
            let organizations = self
                .organizations
                .lock()
                .map_err(|_| OrganizationRepositoryError::Unavailable)?;

            Ok(organizations.get(&organization_id).cloned())
        }
    }

    struct FailingOrganizationRepository;

    #[async_trait]
    impl OrganizationRepository for FailingOrganizationRepository {
        async fn insert(
            &self,
            _organization: Organization,
        ) -> Result<(), OrganizationRepositoryError> {
            Err(OrganizationRepositoryError::Unavailable)
        }

        async fn find_by_id(
            &self,
            _organization_id: OrganizationId,
        ) -> Result<Option<Organization>, OrganizationRepositoryError> {
            Err(OrganizationRepositoryError::Unavailable)
        }
    }

    fn command(owner_user_id: UserId) -> CreateOrganization {
        CreateOrganization {
            name: "ORBYN".to_owned(),
            owner_user_id,
            created_at_unix_ms: 1_000,
        }
    }

    #[tokio::test]
    async fn creates_and_retrieves_an_organization() {
        let application = OrganizationApplication::new(InMemoryOrganizationRepository::default());
        let owner_user_id = UserId(Uuid::new_v4());
        let created = application
            .create(command(owner_user_id))
            .await
            .map_err(|error| error.to_string());

        assert!(created.is_ok());

        if let Ok(organization) = created {
            let found = application.find(organization.id()).await;
            assert!(matches!(found, Ok(Some(found)) if found == organization));
        }
    }

    #[tokio::test]
    async fn rejects_invalid_domain_data_before_persisting() {
        let application = OrganizationApplication::new(InMemoryOrganizationRepository::default());
        let mut invalid_command = command(UserId(Uuid::new_v4()));
        invalid_command.name = "  ".to_owned();

        let result = application.create(invalid_command).await;

        assert!(matches!(
            result,
            Err(OrganizationApplicationError::Domain(
                DomainError::InvalidName
            ))
        ));
    }

    #[tokio::test]
    async fn translates_repository_failures() {
        let application = OrganizationApplication::new(FailingOrganizationRepository);
        let result = application.create(command(UserId(Uuid::new_v4()))).await;

        assert!(matches!(
            result,
            Err(OrganizationApplicationError::Repository(
                OrganizationRepositoryError::Unavailable
            ))
        ));
    }

    #[derive(Default)]
    struct InMemoryExternalIdentityRepository {
        identities: Mutex<Vec<ExternalIdentity>>,
    }

    #[async_trait]
    impl ExternalIdentityRepository for InMemoryExternalIdentityRepository {
        async fn insert(
            &self,
            identity: ExternalIdentity,
        ) -> Result<(), ExternalIdentityRepositoryError> {
            let mut identities = self
                .identities
                .lock()
                .map_err(|_| ExternalIdentityRepositoryError::Unavailable)?;

            if identities.iter().any(|existing| {
                existing.provider() == identity.provider()
                    && existing.external_account_id() == identity.external_account_id()
            }) {
                return Err(ExternalIdentityRepositoryError::Conflict);
            }

            identities.push(identity);
            Ok(())
        }

        async fn find_by_external_account(
            &self,
            provider: IdentityProvider,
            external_account_id: &ExternalAccountId,
        ) -> Result<Option<ExternalIdentity>, ExternalIdentityRepositoryError> {
            let identities = self
                .identities
                .lock()
                .map_err(|_| ExternalIdentityRepositoryError::Unavailable)?;

            Ok(identities
                .iter()
                .find(|identity| {
                    identity.provider() == provider
                        && identity.external_account_id() == external_account_id
                })
                .cloned())
        }
    }

    struct FailingExternalIdentityRepository;

    #[async_trait]
    impl ExternalIdentityRepository for FailingExternalIdentityRepository {
        async fn insert(
            &self,
            _identity: ExternalIdentity,
        ) -> Result<(), ExternalIdentityRepositoryError> {
            Err(ExternalIdentityRepositoryError::Unavailable)
        }

        async fn find_by_external_account(
            &self,
            _provider: IdentityProvider,
            _external_account_id: &ExternalAccountId,
        ) -> Result<Option<ExternalIdentity>, ExternalIdentityRepositoryError> {
            Err(ExternalIdentityRepositoryError::Unavailable)
        }
    }

    fn link_command(user_id: UserId) -> LinkExternalIdentity {
        LinkExternalIdentity {
            user_id,
            provider: IdentityProvider::GitHub,
            external_account_id: "987654".to_owned(),
            display_name: Some("orbyn-dev".to_owned()),
        }
    }

    #[tokio::test]
    async fn links_and_resolves_an_external_identity() {
        let application =
            ExternalIdentityApplication::new(InMemoryExternalIdentityRepository::default());
        let user_id = UserId(Uuid::new_v4());
        let linked = application.link(link_command(user_id)).await;

        assert!(matches!(linked, Ok(identity) if identity.user_id() == user_id));

        let found = application.find(IdentityProvider::GitHub, "987654").await;

        assert!(matches!(found, Ok(Some(identity)) if identity.user_id() == user_id));
    }

    #[tokio::test]
    async fn rejects_an_external_identity_already_linked() {
        let application =
            ExternalIdentityApplication::new(InMemoryExternalIdentityRepository::default());
        let first_user = UserId(Uuid::new_v4());
        let second_user = UserId(Uuid::new_v4());

        let first = application.link(link_command(first_user)).await;
        let second = application.link(link_command(second_user)).await;

        assert!(first.is_ok());
        assert!(matches!(
            second,
            Err(ExternalIdentityApplicationError::Repository(
                ExternalIdentityRepositoryError::Conflict
            ))
        ));
    }

    #[tokio::test]
    async fn rejects_invalid_external_account_ids() {
        let application =
            ExternalIdentityApplication::new(InMemoryExternalIdentityRepository::default());
        let mut command = link_command(UserId(Uuid::new_v4()));
        command.external_account_id = "  ".to_owned();

        let result = application.link(command).await;

        assert!(matches!(
            result,
            Err(ExternalIdentityApplicationError::Identity(
                IdentityError::InvalidExternalAccountId
            ))
        ));
    }

    #[tokio::test]
    async fn translates_external_identity_repository_failures() {
        let application = ExternalIdentityApplication::new(FailingExternalIdentityRepository);
        let result = application.link(link_command(UserId(Uuid::new_v4()))).await;

        assert!(matches!(
            result,
            Err(ExternalIdentityApplicationError::Repository(
                ExternalIdentityRepositoryError::Unavailable
            ))
        ));
    }
}
