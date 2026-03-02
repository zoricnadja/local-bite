#[derive(Debug, PartialEq, Clone)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING"   => Some(Self::Pending),
            "CONFIRMED" => Some(Self::Confirmed),
            "SHIPPED"   => Some(Self::Shipped),
            "DELIVERED" => Some(Self::Delivered),
            "CANCELLED" => Some(Self::Cancelled),
            _           => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending   => "PENDING",
            Self::Confirmed => "CONFIRMED",
            Self::Shipped   => "SHIPPED",
            Self::Delivered => "DELIVERED",
            Self::Cancelled => "CANCELLED",
        }
    }

    /// Returns the set of statuses this status can transition to.
    pub fn allowed_transitions(&self) -> &[OrderStatus] {
        match self {
            Self::Pending   => &[Self::Confirmed, Self::Cancelled],
            Self::Confirmed => &[Self::Shipped,   Self::Cancelled],
            Self::Shipped   => &[Self::Delivered],
            Self::Delivered => &[],
            Self::Cancelled => &[],
        }
    }

    pub fn can_transition_to(&self, next: &OrderStatus) -> bool {
        self.allowed_transitions().contains(next)
    }
}
