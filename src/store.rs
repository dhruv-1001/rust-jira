use crate::models::{Comment, DeletedTicket, Status, Ticket, TicketDraft, TicketId, TicketPatch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TicketStore {
    current_id: u64,
    data: HashMap<TicketId, Ticket>,
}


impl TicketStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn create(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            comments: Vec::new()
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn delete(&mut self, ticket_id: TicketId) -> Option<DeletedTicket> {
        self.data.remove(&ticket_id).map(DeletedTicket)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.iter().map(|(_, ticket)| ticket).collect()
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.data.get(&id)
    }

    pub fn update_ticket(&mut self, id: TicketId, patch: TicketPatch) -> Option<()> {
        self.data.get_mut(&id).map(|t| {
            if let Some(title) = patch.title {
                t.title = title;
            }
            if let Some(description) = patch.description {
                t.description = description;
            }
        })
    }

    pub fn update_ticket_status(&mut self, id: TicketId, status: Status) -> Option<()> {
        self.data.get_mut(&id).map(|t| t.status = status)
    }

    pub fn add_comment_to_ticket(&mut self, id: TicketId, comment: Comment) -> Option<()> {
        self.data.get_mut(&id).map(|t| t.comments.push(comment))
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}