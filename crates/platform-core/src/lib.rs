//! Small, dependency-free foundational types for ORBYN.

/// Package identity for diagnostics.
pub const PACKAGE_NAME: &str = "platform-core";

#[cfg(test)]
mod tests {
    use super::PACKAGE_NAME;

    #[test]
    fn exposes_package_identity() {
        assert_eq!(PACKAGE_NAME, "platform-core");
    }
}
