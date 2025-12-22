use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::user_management::domain::entities::entity::Entity;

#[derive(Debug, Clone, Eq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Entity for User {
    fn id(&self) -> Uuid {
        self.id
    }
}
impl User {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }

    pub fn from_existing(id: Uuid, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}
