use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    SystemAdmin,
    FarmOwner,
    Worker,
    Customer,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::SystemAdmin => "SYSTEM_ADMIN",
            Role::FarmOwner   => "FARM_OWNER",
            Role::Worker      => "WORKER",
            Role::Customer    => "CUSTOMER",
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
            "FARM_OWNER"   => Ok(Role::FarmOwner),
            "WORKER"       => Ok(Role::Worker),
            "CUSTOMER"     => Ok(Role::Customer),
            _ => Err(anyhow::anyhow!("Invalid role: {}", s)),
        }
    }
}