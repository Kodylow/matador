// //! Simple Model Layer
// //! (with mock-store layer)

// use crate::{ctx::Ctx, Error, Result};
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, Mutex};

// // region: -- Ticket Types

// #[derive(Debug, Clone, Serialize)]
// pub struct Ticket {
//     pub id: u64,
//     pub cid: u64, // creator user id
//     pub title: String,
// }

// #[derive(Deserialize, Debug)]
// pub struct TicketForCreate {
//     pub title: String,
// }

// // endregion: -- Ticket Types

// // region: -- Model Controller

// #[derive(Clone)]
// pub struct ModelController {
//     tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
// }

// // Constructor
// impl ModelController {
//     pub async fn new() -> Result<Self> {
//         Ok(Self {
//             tickets_store: Arc::default(),
//         })
//     }
// }

// // CRUD Implementation
// impl ModelController {
//     pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketForCreate) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//         let id = store.len() as u64; // works because Rust guarantees exclusive access to the store
//         let ticket = Ticket {
//             id,
//             cid: ctx.user_id(),
//             title: ticket.title,
//         };
//         store.push(Some(ticket.clone()));
//         Ok(ticket)
//     }

//     pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
//         let store = self.tickets_store.lock().unwrap();
//         let tickets = store
//             .iter()
//             .filter_map(|ticket| ticket.clone()) // filter out the None values
//             .collect::<Vec<_>>();
//         Ok(tickets)
//     }

//     pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
//         let mut store = self.tickets_store.lock().unwrap();
//         let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take()); // take the value out of the Option

//         ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
//     }
// }

// // endregion: -- Model Controller
