use crate::models::{Comment, Title};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Error;
use std::fmt::Formatter;

pub type TicketId = u64;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: Title,
    pub description: String,
    pub status: Status,
    pub comments: Vec<Comment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(
            f,
            "Ticket:\n\tId:{:?}\n\tTitle:{}\n\tDescription:{}\n\tStatus:{:?}\n\tComments:",
            self.id, self.title, self.description, self.status
        )?;
        for comment in self.comments.iter() {
            writeln!(f, "\t- {}", comment)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

pub struct DeletedTicket(pub Ticket);
