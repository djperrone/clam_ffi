#[repr(C)]
#[derive(Debug)]
pub enum FFIError {
    /// All went fine.
    Ok,

    /// Naughty API call detected.
    NullPointerPassed = 1,
    InvalidStringPassed = 2,
    HandleInitFailed = 3,
    GraphBuildFailed = 4,
    QueryIsNull,
    PhysicsAlreadyShutdown,
}
