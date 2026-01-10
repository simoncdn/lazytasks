use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Priority;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub completed: bool,
    pub archived: bool,
    pub description: String,
    pub priority: Option<Priority>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        return Task {
            id: Uuid::new_v4(),
            title: title.into(),
            created_at: Utc::now(),
            updated_at: None,
            archived_at: None,
            completed: false,
            archived: false,
            description: String::new(),
            priority: None,
        };
    }
}
