pub mod jwt;
pub mod password_hasher;

// Used for interface to communicate with middleware for
// type validation, expecially on sceniorous where certain roles permitted
pub trait PermissionIntance {
    type AsPermission: PartialEq + 'static;
    fn permission_ref(&self) -> &Self::AsPermission;
}
