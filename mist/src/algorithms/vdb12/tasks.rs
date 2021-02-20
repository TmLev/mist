use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Task {
    deadline: DateTime<Utc>,
}

impl Task {
    pub fn new(deadline: DateTime<Utc>) -> Self {
        Self { deadline }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            deadline: Utc::now(),
        }
    }
}
