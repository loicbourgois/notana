use uuid::Uuid;
#[derive(Debug)]
pub enum TaskStatus {
    New,
    Done,
}
#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub subtasks: Vec<Uuid>,
    pub status: TaskStatus,
    pub id: Uuid,
}
