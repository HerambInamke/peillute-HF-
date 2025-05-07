use super::db;
use rusqlite::Connection;
use std::io::{self as std_io, Write};
use serde::{Deserialize, Serialize};
use crate::{message::CreateUser, network::send_message_to_all};
use std::error::Error;
use crate::message::MessageInfo;

// renvoie une commande 
pub fn run_cli(
    line: Result<Option<String>, std::io::Error>,
) -> Command {
    match line {
        Ok(Some(cmd)) => {
            let command = parse_command(&cmd);

            print!("> ");
            std_io::stdout().flush().unwrap();
            command
        }
        Ok(None) => {
            log::info!("Aucun input");
            Command::Unknown("Aucun input".to_string())
        }
        Err(e) => {
            log::error!("Erreur de lecture stdin : {}", e);
            Command::Error("Erreur de lecture stdin".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    CreateUser,
    UserAccounts,
    PrintUserTransactions,
    PrintTransactions,
    Deposit,
    Withdraw,
    Transfer,
    Pay,
    Refund,
    Help,
    Unknown(String),
    Error(String),
}

fn parse_command(input: &str) -> Command {
    match input.trim() {
        "/create_user" => Command::CreateUser,
        "/user_accounts" => Command::UserAccounts,
        "/print_user_tsx" => Command::PrintUserTransactions,
        "/print_tsx" => Command::PrintTransactions,
        "/deposit" => Command::Deposit,
        "/withdraw" => Command::Withdraw,
        "/transfer" => Command::Transfer,
        "/pay" => Command::Pay,
        "/refund" => Command::Refund,
        "/help" => Command::Help,
        other => Command::Unknown(other.to_string()),
    }
}

pub async fn handle_command(cmd: Command, conn: &Connection, lamport_time: &mut i64, node: &str, from_network : bool)-> Result<(), Box<dyn Error>> {

    
    match cmd {
        Command::CreateUser => {
            let name = prompt("Username");
            db::create_user(conn, &name).unwrap();
            if !from_network {
                let _ = send_message_to_all(
                    Some(Command::CreateUser),
                    crate::message::NetworkMessageCode::Transaction,
                    crate::message::MessageInfo::CreateUser(CreateUser::new(name.clone())),
                )
                .await?;
            }
        }

        Command::UserAccounts => {
            db::print_users(conn).unwrap();
        }

        Command::PrintUserTransactions => {
            let name = prompt("Username");
            db::print_transaction_for_user(conn, &name).unwrap();
        }

        Command::PrintTransactions => {
            db::print_transactions(conn).unwrap();
        }

        Command::Deposit => {
            let name = prompt("Username");
            let amount = prompt_parse::<f64>("Deposit amount");
            db::deposit(conn, &name, amount, lamport_time, node).unwrap();
            if !from_network {
                let _ = send_message_to_all(
                    Some(Command::Deposit),
                    crate::message::NetworkMessageCode::Transaction,
                    crate::message::MessageInfo::Deposit(crate::message::Deposit::new(name.clone(), amount)),
                )
                .await?;
            }
        }

        Command::Withdraw => {
            let name = prompt("Username");
            let amount = prompt_parse::<f64>("Withdraw amount");
            db::withdraw(conn, &name, amount, lamport_time, node).unwrap();
            if !from_network {
                let _ = send_message_to_all(
                    Some(Command::Withdraw),
                    crate::message::NetworkMessageCode::Transaction,
                    crate::message::MessageInfo::Withdraw(crate::message::Withdraw::new(name.clone(), amount)),
                )
                .await?;
            }

        }

        Command::Transfer => {
            let name = prompt("Username");
            let amount = prompt_parse::<f64>("Transfer amount");
            let _ = db::print_users(conn);
            let beneficiary = prompt("Beneficiary");
            db::create_transaction(conn, &name, &beneficiary, amount, lamport_time, node, "")
                .unwrap();

            if !from_network {
                let _ = send_message_to_all(
                    Some(Command::Transfer),
                    crate::message::NetworkMessageCode::Transaction,
                    crate::message::MessageInfo::Transfer(crate::message::Transfer::new(name.clone(), beneficiary.clone(), amount)),
                )
                .await?;
            }

        }

        Command::Pay => {
            let name = prompt("Username");
            let amount = prompt_parse::<f64>("Payment amount");
            db::create_transaction(conn, &name, "NULL", amount, lamport_time, node, "").unwrap();
            
            if !from_network {
                let _ = send_message_to_all(
                    Some(Command::Pay),
                    crate::message::NetworkMessageCode::Transaction,
                    crate::message::MessageInfo::Pay(crate::message::Pay::new(name.clone(), amount)),
                )
                .await?;
            }

        }

        Command::Refund => {
            let name = prompt("Username");
            db::print_transaction_for_user(conn, &name).unwrap();
            let transac_time = prompt_parse::<i64>("Lamport time");
            let transac_node = prompt("Node");
            db::refund_transaction(conn, transac_time, &transac_node, lamport_time, node).unwrap();

            if !from_network {
               // TODO : send message
            }

        }

        Command::Help => {
            log::info!("📜 Command list:");
            log::info!("/create_user      - Create a personal account");
            log::info!("/user_accounts    - List all users");
            log::info!("/print_user_tsx   - Show a user’s transactions");
            log::info!("/print_tsx        - Show all system transactions");
            log::info!("/deposit          - Deposit money to an account");
            log::info!("/withdraw         - Withdraw money from an account");
            log::info!("/transfer         - Transfer money to another user");
            log::info!("/pay              - Make a payment (to NULL)");
            log::info!("/refund           - Refund a transaction");
        }

        Command::Unknown(cmd) => {
            log::info!("❓ Unknown command: {}", cmd);
        }

        Command::Error(err) => {
            log::error!("❌ Error: {}", err);
        }
    }
    Ok(())
}

fn prompt(label: &str) -> String {
    let mut input = String::new();
    print!("{label} > ");
    std_io::stdout().flush().unwrap();
    std_io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_parse<T: std::str::FromStr>(label: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    loop {
        let input = prompt(label);
        match input.parse::<T>() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Try again."),
        }
    }
}
