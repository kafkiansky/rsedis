use crate::client::{Client, Conf};
use crate::proto::Command;

fn main() {
    if let Ok(mut client) = Client::connect(Conf {
        host: "127.0.0.1:6379".to_string(),
        password: "xxxx".to_string(),
        database: 0,
    }) {
        client.command(Command::set(String::from("x"), "y"))
    }
}
