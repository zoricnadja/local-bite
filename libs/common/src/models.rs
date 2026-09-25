use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    SystemAdmin,
    BusinessOwner,
    Worker,
    Customer,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::SystemAdmin => "SYSTEM_ADMIN",
            Role::BusinessOwner => "BUSINESS_OWNER",
            Role::Worker => "WORKER",
            Role::Customer => "CUSTOMER",
        }
    }
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(Role::Customer)
    }
}

impl FromStr for Role {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SYSTEM_ADMIN" => Ok(Role::SystemAdmin),
            "BUSINESS_OWNER" => Ok(Role::BusinessOwner),
            "WORKER" => Ok(Role::Worker),
            "CUSTOMER" => Ok(Role::Customer),
            _ => Err(anyhow::anyhow!("Invalid role: {}", s)),
        }
    }
}
