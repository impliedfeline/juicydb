use juicydb::parser::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to juicydb");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.is_empty() {
            break;
        }

        let mut parser = Parser::new(&input);
        let stmt = parser.parse_command();

        match stmt {
            Ok(stmt) => match stmt {
                Command::Statement(stmt) => println!("{:#?}", stmt),
                Command::MetaCommand(cmd) => {
                    match cmd {
                        MetaCommand::Exit => return,
                        MetaCommand::Print => todo!(),
                    }
                }
            },
            Err(err) => println!("Error: {}", err),
        }
    }
}
