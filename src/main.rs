use juicydb::db::*;
use juicydb::parser::*;
use juicydb::storage_manager::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to juicydb");

    let mut storage = StorageManager::new();

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
                Command::Statement(stmt) => {
                    let process = match stmt {
                        Statement::CreateTable { table, columns } => {
                            storage.create_table(table, Schema::from(columns))
                        }
                        Statement::InsertInto { table, values } => {
                            storage.insert_into(table, values)
                        }
                        query => storage.query(query).and_then(|rows| {
                            for row in rows {
                                for col in row {
                                    print!("{}, ", col);
                                }
                                println!();
                            }
                            Ok(())
                        }),
                    };
                    if let Err(err) = process {
                        println!("SQL error: {}", err);
                    };
                }
                Command::MetaCommand(cmd) => match cmd {
                    MetaCommand::Exit => return,
                    MetaCommand::Print => println!("{:#?}", storage),
                },
            },
            Err(err) => println!("Parse error: {}", err),
        };
    }
}
