/*
 * Copyright (c) 2023-present repelDB
 *
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use serde::{
    Deserialize,
    Serialize,
};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub perm: UserPermission,
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

    pub fn compare(
        &self,
        o: &UserPermission,
    ) -> bool {
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
