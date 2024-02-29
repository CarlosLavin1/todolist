#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}