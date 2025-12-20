#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(id: usize, title: String, description: String) -> Self {
        return Task {
            id,
            title,
            description,
        };
    }
}
