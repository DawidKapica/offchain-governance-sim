use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProposalStatus {
    Open,
    Closed,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub votes_yes: u32,
    pub votes_no: u32,
    pub status: ProposalStatus,
}

impl Proposal {
    pub fn new(id: u32, title: String) -> Self {
        Proposal {
            id,
            title,
            votes_yes: 0,
            votes_no: 0,
            status: ProposalStatus::Open,
            created_at: Utc::now(),
        }
    }

    pub fn record_vote(&mut self, weight: u32, support: bool) {
        if support {
            self.votes_yes += weight;
        } else {
            self.votes_no += weight;
        }
    }

    pub fn result(&self) -> String {
        println!("Created at: {}", self.created_at);
        println!("Status: {:?}", self.status);

        let total = self.votes_yes + self.votes_no;
        if total == 0 {
            "No votes yet".into()
        } else if self.votes_yes > self.votes_no {
            format!("Accepted ({} yes / {} no)", self.votes_yes, self.votes_no)
        } else {
            format!("Rejected ({} yes / {} no)", self.votes_yes, self.votes_no)
        }
    }

    pub fn close(&mut self) {
        self.status = ProposalStatus::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_logic() {
        let mut p = Proposal::new(1, "Test Proposal".to_string());
        p.record_vote(2, true);
        p.record_vote(1, false);
        assert_eq!(p.votes_yes, 2);
        assert_eq!(p.votes_no, 1);
    }

    #[test]
    fn test_result_output() {
        let mut p = Proposal::new(2, "Another".to_string());
        p.record_vote(3, true);
        assert!(p.result().contains("Accepted"));
        p.record_vote(5, false);
        assert!(p.result().contains("Rejected"));
    }

    #[test]
    fn test_status_change() {
        let mut p = Proposal::new(3, "Closeable".to_string());
        assert!(matches!(p.status, ProposalStatus::Open));
        p.close();
        assert!(matches!(p.status, ProposalStatus::Closed));
    }
}
