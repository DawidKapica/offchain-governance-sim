use crate::proposal::{Proposal, ProposalStatus};
use crate::user::{Role, User};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub users: HashMap<u32, User>,
    pub proposals: HashMap<u32, Proposal>,
    pub next_user_id: u32,
    pub next_prop_id: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            users: HashMap::new(),
            proposals: HashMap::new(),
            next_user_id: 1,
            next_prop_id: 1,
        }
    }

    pub fn add_user(&mut self, name: String, role: Role) {
        let id = self.next_user_id;
        let user = User::new(id, name, role);
        self.users.insert(id, user);
        self.next_user_id += 1;
    }

    pub fn add_proposal(&mut self, title: String) {
        let id = self.next_prop_id;
        let prop = Proposal::new(id, title);
        self.proposals.insert(id, prop);
        self.next_prop_id += 1;
    }

    pub fn close_proposal(&mut self, prop_id: u32) {
        if let Some(prop) = self.proposals.get_mut(&prop_id) {
            prop.close();
            println!("Proposal #{} closed.", prop_id);
        } else {
            println!("Proposal not found.");
        }
    }

    pub fn vote(&mut self, user_id: u32, prop_id: u32, support: bool) {
        if let (Some(user), Some(prop)) = (
            self.users.get(&user_id),
            self.proposals.get_mut(&prop_id)
        ) {
            if let ProposalStatus::Open = prop.status {
                prop.record_vote(user.role.weight(), support);
                println!("Vote recorded.");
            } else {
                println!("Voting is closed for this proposal.");
            }
        }
    }

    pub fn export(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Serialization failed");
        fs::write(path, json).expect("Unable to write file");
    }

    pub fn import(&mut self, path: &str) {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let loaded: State = serde_json::from_str(&data).expect("Deserialization failed");
        *self = loaded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::Role;

    #[test]
    fn test_add_user_and_proposal() {
        let mut state = State::new();
        state.add_user("Alice".to_string(), Role::User);
        state.add_proposal("Proposal 1".to_string());
        assert_eq!(state.users.len(), 1);
        assert_eq!(state.proposals.len(), 1);
    }

    #[test]
    fn test_vote_flow() {
        let mut state = State::new();
        state.add_user("Bob".to_string(), Role::Validator);
        state.add_proposal("Change fees".to_string());
        state.vote(1, 1, true);

        let prop = state.proposals.get(&1).unwrap();
        assert_eq!(prop.votes_yes, 3);
        assert_eq!(prop.votes_no, 0);
    }

    #[test]
    fn test_close_proposal() {
        let mut state = State::new();
        state.add_proposal("Temporary".to_string());
        let prop = state.proposals.get(&1).unwrap();
        assert!(matches!(prop.status, ProposalStatus::Open));

        state.close_proposal(1);
        let prop_closed = state.proposals.get(&1).unwrap();
        assert!(matches!(prop_closed.status, ProposalStatus::Closed));
    }
}
