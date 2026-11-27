use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TeamId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkflowId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("el nombre debe contener entre 1 y 100 caracteres")]
    InvalidName,
    #[error("la marca de tiempo no puede ser negativa")]
    InvalidTimestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Name(String);

impl Name {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let normalized = value.trim();

        if normalized.is_empty() || normalized.chars().count() > 100 {
            return Err(DomainError::InvalidName);
        }

        Ok(Self(normalized.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Name {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    id: OrganizationId,
    name: Name,
    owner_user_id: UserId,
    created_at_unix_ms: i64,
}

impl Organization {
    pub fn new(
        name: impl Into<String>,
        owner_user_id: UserId,
        created_at_unix_ms: i64,
    ) -> Result<Self, DomainError> {
        validate_timestamp(created_at_unix_ms)?;

        Ok(Self {
            id: OrganizationId(Uuid::new_v4()),
            name: Name::new(name)?,
            owner_user_id,
            created_at_unix_ms,
        })
    }

    pub fn id(&self) -> OrganizationId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn owner_user_id(&self) -> UserId {
        self.owner_user_id
    }

    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    display_name: Name,
    created_at_unix_ms: i64,
}

impl User {
    pub fn new(
        display_name: impl Into<String>,
        created_at_unix_ms: i64,
    ) -> Result<Self, DomainError> {
        validate_timestamp(created_at_unix_ms)?;

        Ok(Self {
            id: UserId(Uuid::new_v4()),
            display_name: Name::new(display_name)?,
            created_at_unix_ms,
        })
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn display_name(&self) -> &Name {
        &self.display_name
    }

    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    id: TeamId,
    organization_id: OrganizationId,
    name: Name,
    created_at_unix_ms: i64,
}

impl Team {
    pub fn new(
        organization_id: OrganizationId,
        name: impl Into<String>,
        created_at_unix_ms: i64,
    ) -> Result<Self, DomainError> {
        validate_timestamp(created_at_unix_ms)?;

        Ok(Self {
            id: TeamId(Uuid::new_v4()),
            organization_id,
            name: Name::new(name)?,
            created_at_unix_ms,
        })
    }

    pub fn id(&self) -> TeamId {
        self.id
    }

    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipRole {
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Membership {
    organization_id: OrganizationId,
    user_id: UserId,
    role: MembershipRole,
    created_at_unix_ms: i64,
}

impl Membership {
    pub fn new(
        organization_id: OrganizationId,
        user_id: UserId,
        role: MembershipRole,
        created_at_unix_ms: i64,
    ) -> Result<Self, DomainError> {
        validate_timestamp(created_at_unix_ms)?;

        Ok(Self {
            organization_id,
            user_id,
            role,
            created_at_unix_ms,
        })
    }

    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn role(&self) -> MembershipRole {
        self.role
    }

    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    organization_id: OrganizationId,
    name: Name,
    created_at_unix_ms: i64,
}

impl Project {
    pub fn new(
        organization_id: OrganizationId,
        name: impl Into<String>,
        created_at_unix_ms: i64,
    ) -> Result<Self, DomainError> {
        validate_timestamp(created_at_unix_ms)?;

        Ok(Self {
            id: ProjectId(Uuid::new_v4()),
            organization_id,
            name: Name::new(name)?,
            created_at_unix_ms,
        })
    }

    pub fn id(&self) -> ProjectId {
        self.id
    }

    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn created_at_unix_ms(&self) -> i64 {
        self.created_at_unix_ms
    }
}

fn validate_timestamp(timestamp: i64) -> Result<(), DomainError> {
    if timestamp < 0 {
        return Err(DomainError::InvalidTimestamp);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_trims_surrounding_whitespace() {
        let name = Name::new("  ORBYN  ").expect("nombre válido");

        assert_eq!(name.as_str(), "ORBYN");
    }

    #[test]
    fn name_rejects_empty_values() {
        assert_eq!(Name::new("   "), Err(DomainError::InvalidName));
    }

    #[test]
    fn name_rejects_values_longer_than_one_hundred_characters() {
        assert_eq!(
            Name::new("a".repeat(101)),
            Err(DomainError::InvalidName)
        );
    }

    #[test]
    fn organization_creates_a_typed_identity() {
        let owner = UserId(Uuid::new_v4());
        let organization = Organization::new("Equipo", owner, 1_000).expect("organización válida");

        assert_eq!(organization.owner_user_id(), owner);
        assert_eq!(organization.name().as_str(), "Equipo");
    }

    #[test]
    fn user_rejects_negative_creation_timestamp() {
        assert_eq!(
            User::new("Desarrollador", -1),
            Err(DomainError::InvalidTimestamp)
        );
    }

    #[test]
    fn team_belongs_to_an_organization() {
        let organization_id = OrganizationId(Uuid::new_v4());
        let team = Team::new(organization_id, "Backend", 1_000).expect("equipo válido");

        assert_eq!(team.organization_id(), organization_id);
    }

    #[test]
    fn membership_keeps_its_organization_role() {
        let organization_id = OrganizationId(Uuid::new_v4());
        let user_id = UserId(Uuid::new_v4());
        let membership = Membership::new(
            organization_id,
            user_id,
            MembershipRole::Admin,
            1_000,
        )
        .expect("membresía válida");

        assert_eq!(membership.role(), MembershipRole::Admin);
        assert_eq!(membership.user_id(), user_id);
    }

    #[test]
    fn project_rejects_negative_creation_timestamp() {
        let organization_id = OrganizationId(Uuid::new_v4());

        assert_eq!(
            Project::new(organization_id, "Repositorio", -1),
            Err(DomainError::InvalidTimestamp)
        );
    }
}
