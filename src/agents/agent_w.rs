use serde::{Deserialize, Serialize};
use yew::worker::{Agent, AgentLink, HandlerId, Context};
use std::collections::HashSet;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetStateCell(usize, usize, String),
    Shot(usize, usize),
    AutoShot(usize),
    ChangeOfTurn,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Response {
    ResultStateCell(usize, usize, String),
    ToggleCell(usize, usize),
    LeaderChange,
}

pub struct Worker {
    link: AgentLink<Worker>,
    subscribers: HashSet<HandlerId>,
}

impl Worker {

    fn send_result_state_cell(&self, idx_cell: usize, idx_field: usize, text: &str) {
        for sub in self.subscribers.iter() {
            if sub.is_respondable() {
                self.link.respond(
                    *sub,
                    Response::ResultStateCell(idx_cell, idx_field, text.to_string()),
                );
            }
        }
    }
    pub fn send_toggle_cell(&self, idx_cell: usize, idx_field: usize) {
        for sub in self.subscribers.iter() {
            if sub.is_respondable() {
                self.link.respond(
                    *sub,
                    Response::ToggleCell(idx_cell, idx_field),
                );
            }
        }
    }
    pub fn send_leader_change(&self) {
        for sub in self.subscribers.iter() {
            if sub.is_respondable() {
                self.link.respond(
                    *sub,
                    Response::LeaderChange,
                );
            }
        }
    }
}

impl Agent for Worker {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            Request::GetStateCell(idx_cell, field_number, text_state) => {
                self.send_result_state_cell(idx_cell, field_number, text_state.as_str());
            }
            Request::Shot(idx_shot, field_number) => {
                self.send_toggle_cell(idx_shot, field_number);
            }
            Request::AutoShot(_) => {
                let mut rng = rand::thread_rng();
                let _idx: usize = rng.gen_range(0, 99);
            }
            Request::ChangeOfTurn => {
                self.send_leader_change();
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}