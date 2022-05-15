use chrono::{DateTime, Utc};

pub struct Ticket{
    id: TicketId,
    title: String,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub enum Status {
    ToDo,
    InProgress,
    Done
}

pub type TicketId = u32;