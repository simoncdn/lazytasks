#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(id: String, title: String, description: String) -> Self {
        return Task {
            id,
            title,
            description,
        };
    }
}
