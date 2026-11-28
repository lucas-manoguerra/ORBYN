use async_trait::async_trait;
use orbyn_domain::{DomainError, Organization, OrganizationId, UserId};
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
    async fn insert(
        &self,
        organization: Organization,
    ) -> Result<(), OrganizationRepositoryError>;

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
        let application = OrganizationApplication::new(
            InMemoryOrganizationRepository::default(),
        );
        let owner_user_id = UserId(Uuid::new_v4());
        let created = application
            .create(command(owner_user_id))
            .await
            .map_err(|error| error.to_string());

        assert!(created.is_ok());
        let organization = match created {
            Ok(organization) => organization,
            Err(error) => panic!("{error}"),
        };

        let found = application.find(organization.id()).await;
        assert_eq!(found, Ok(Some(organization)));
    }

    #[tokio::test]
    async fn rejects_invalid_domain_data_before_persisting() {
        let application = OrganizationApplication::new(
            InMemoryOrganizationRepository::default(),
        );
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
}
