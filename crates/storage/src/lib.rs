pub trait HealthCheck: Send + Sync {
    fn is_ready(&self) -> bool;
}
