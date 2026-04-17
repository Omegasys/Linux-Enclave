#[derive(Debug, PartialEq)]
pub enum LifecycleState {
    Created,
    Running,
    Suspended,
    Stopped,
}
