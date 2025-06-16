mod governance;
mod proposal;
mod user;

use crate::governance::State;
use crate::user::Role;
use std::io;

fn main() {
    let mut state = State::new();

    loop {
        println!(
            "\nOptions:\n1) Add user\n2) Add proposal\n3) Vote\n4) Show results\n5) Close proposal\n6) Export state\n7) Import state\n8) Exit"
        );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter user name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Select role (user, validator, admin):");
                let mut r = String::new();
                io::stdin().read_line(&mut r).unwrap();
                let role = match r.trim() {
                    "validator" => Role::Validator,
                    "admin" => Role::Admin,
                    _ => Role::User,
                };
                state.add_user(name.trim().to_string(), role);
                println!("User added. Current total: {}", state.users.len());
            }
            "2" => {
                println!("Enter proposal title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                state.add_proposal(title.trim().to_string());
                println!("Proposal added. Total proposals: {}", state.proposals.len());
            }
            "3" => {
                println!("Enter user ID:");
                let mut uid = String::new();
                io::stdin().read_line(&mut uid).unwrap();
                println!("Enter proposal ID:");
                let mut pid = String::new();
                io::stdin().read_line(&mut pid).unwrap();
                println!("Support? (yes/no):");
                let mut sup = String::new();
                io::stdin().read_line(&mut sup).unwrap();

                if let (Ok(u), Ok(p)) = (uid.trim().parse(), pid.trim().parse()) {
                    let support = sup.trim() == "yes";
                    state.vote(u, p, support);
                }
            }
            "4" => {
                for prop in state.proposals.values() {
                    println!("\n#{}: {} ⇒ {}", prop.id, prop.title, prop.result());
                }
            }
            "5" => {
                println!("Enter proposal ID to close:");
                let mut pid = String::new();
                io::stdin().read_line(&mut pid).unwrap();
                if let Ok(id) = pid.trim().parse::<u32>() {
                    state.close_proposal(id);
                }
            }
            "6" => {
                state.export("state.json");
                println!("State exported to state.json");
            }
            "7" => {
                state.import("state.json");
                println!("State imported from state.json");
            }
            "8" => break,

            _ => println!("Invalid option, try again."),
        }
    }
}
