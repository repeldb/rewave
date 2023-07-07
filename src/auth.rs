use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserPermission {
    Read,
    Write,
    ReadAndWrite,
    Admin,
}

#[derive(Debug)]
pub enum UserPermissionError {
    InvalidPermission,
}

impl UserPermission {
    pub fn from_str(s: &str) -> Result<UserPermission, UserPermissionError> {
        match s.to_lowercase().as_str() {
            "read" => Ok(UserPermission::Read),
            "write" => Ok(UserPermission::Write),
            "read-write" => Ok(UserPermission::ReadAndWrite),
            "admin" => Ok(UserPermission::Admin),
            _ => Err(UserPermissionError::InvalidPermission),
        }
    }

    pub fn compare(&self, o: &UserPermission) -> bool {
        match self {
            UserPermission::Read => matches!(o, UserPermission::Read),
            UserPermission::Write => matches!(o, UserPermission::Write),
            UserPermission::ReadAndWrite => matches!(
                o,
                UserPermission::ReadAndWrite | UserPermission::Read | UserPermission::Write
            ),
            UserPermission::Admin => true,
        }
    }
}
