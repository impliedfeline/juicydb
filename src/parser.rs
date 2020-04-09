use crate::db::*;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
pub enum Statement {
    Select {
        columns: Vec<Identifier>,
        table: Identifier,
        condition: Option<Condition>,
    },
    CreateTable {
        table: Identifier,
        columns: Vec<(Identifier, DBType)>,
    },
    InsertInto {
        table: Identifier,
        values: Vec<DBValue>,
    },
}

type Identifier = String;

#[derive(Debug)]
pub enum Condition {
    Literal(ConditionLiteral),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Debug)]
pub struct Selector {
    table: Identifier,
    field: Identifier,
}

#[derive(Debug)]
pub enum ConditionLiteral {
    Eq(Selector, Selector),
    Neq(Selector, Selector),
    Lt(Selector, Selector),
    Lte(Selector, Selector),
    Gt(Selector, Selector),
    Gte(Selector, Selector),
}

#[derive(Debug)]
pub enum MetaCommand {
    Exit,
    Print,
}

#[derive(Debug)]
pub enum Command {
    MetaCommand(MetaCommand),
    Statement(Statement),
}

pub struct Parser<'a> {
    input: &'a str,
}

#[derive(Debug)]
pub enum ParseError {
    FailedToLex,
    InvalidIdentifier,
    InvalidValue,
    EndOfInput,
    MissingSemicolon,
    MissingLParen,
    MissingRParen,
    MissingComma,
    UnrecognizedMetaCommand,
    UnrecognizedStatement,
    UnrecognizedType,
    RunawayText,
    MissingFrom,
    MissingType,
}

impl ParseError {
    fn ignore_fail(self) -> Result<(), ParseError> {
        if let ParseError::FailedToLex = self {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FailedToLex => write!(f, "Failed to lex"),
            Self::InvalidIdentifier => write!(f, "Invalid identifier"),
            Self::EndOfInput => write!(f, "Unexpected end of input"),
            Self::MissingSemicolon => write!(f, "Missing semicolon"),
            Self::MissingLParen => write!(f, "Missing left parenthesis from column list"),
            Self::MissingRParen => write!(f, "Missing right parenthesis from column list"),
            Self::MissingComma => write!(f, "Missing comma from column list"),
            Self::UnrecognizedMetaCommand => write!(f, "Unrecognized meta-command"),
            Self::UnrecognizedStatement => write!(f, "Unrecognized SQL statement"),
            Self::UnrecognizedType => write!(f, "Unrecognized data type"),
            Self::RunawayText => write!(f, "No closing delimiter for text"),
            Self::InvalidValue => write!(f, "Invalid value"),
            Self::MissingFrom => write!(f, "Missing 'from' clause in 'select'-statement"),
            Self::MissingType => write!(f, "Missing type in column list"),
        }
    }
}

enum Sign {
    Positive,
    Negative,
}

fn char_to_i64(input: char) -> i64 {
    match input {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Not a valid digit"),
    }
}

fn str_to_i64(input: &str) -> i64 {
    let mut val = 0;
    for (i, c) in input.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            val += char_to_i64(c) * 10_i64.pow(i.try_into().unwrap());
        } else if c == '-' && i == input.len() - 1 {
            val = -val;
        } else {
            panic!("Not a valid integer");
        }
    }
    val
}

type ParseResult<T> = Result<T, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn skip_whitespace(&mut self) {
        let count = self.input.chars().take_while(|c| c.is_whitespace()).count();
        let (_, input) = self.input.split_at(count);
        self.input = input;
    }

    fn lex_string(&mut self, string: &str) -> ParseResult<()> {
        self.skip_whitespace();
        if self.input.starts_with(string) {
            let (_, input) = self.input.split_at(string.len());
            self.input = input;
            Ok(())
        } else {
            if self.input.is_empty() {
                Err(ParseError::EndOfInput)
            } else {
                Err(ParseError::FailedToLex)
            }
        }
    }

    fn lex_identifier(&mut self) -> ParseResult<Identifier> {
        self.skip_whitespace();
        let mut chars = self.input.chars();
        if let Some(c) = chars.nth(0) {
            if c.is_ascii_alphabetic() {
                let count = 1 + chars
                    .take_while(|&c| c.is_ascii_alphanumeric() || c == '_')
                    .count();
                let (parsed, input) = self.input.split_at(count);
                self.input = input;
                Ok(String::from(parsed))
            } else {
                Err(ParseError::InvalidIdentifier)
            }
        } else {
            Err(ParseError::EndOfInput)
        }
    }

    fn parse_text(&mut self) -> ParseResult<String> {
        let mut chars = self.input.chars();
        if let Some(c) = chars.nth(0) {
            if c == '"' {
                let count = 1 + chars.take_while(|&c| c != '"').count();
                if let Some('"') = self.input.chars().nth(count) {
                    let (parsed, input) = self.input.split_at(count + 1);
                    self.input = input;
                    Ok(String::from(parsed))
                } else {
                    Err(ParseError::RunawayText)
                }
            } else {
                Err(ParseError::FailedToLex)
            }
        } else {
            Err(ParseError::EndOfInput)
        }
    }

    fn parse_positive_integer(&mut self) -> ParseResult<i64> {
        let count = self
            .input
            .chars()
            .take_while(|&c| c.is_ascii_digit())
            .count();
        if count > 0 {
            let (parsed, input) = self.input.split_at(count);
            self.input = input;
            Ok(str_to_i64(parsed))
        } else {
            Err(ParseError::FailedToLex)
        }
    }

    fn parse_negative_integer(&mut self) -> ParseResult<i64> {
        let mut chars = self.input.chars();
        if let Some(c) = chars.nth(0) {
            if c == '-' {
                let count = chars.take_while(|&c| c.is_ascii_digit()).count();
                if count > 0 {
                    let (parsed, input) = self.input.split_at(count + 1);
                    self.input = input;
                    Ok(str_to_i64(parsed))
                } else {
                    Err(ParseError::FailedToLex)
                }
            } else {
                Err(ParseError::FailedToLex)
            }
        } else {
            Err(ParseError::EndOfInput)
        }
    }

    fn parse_integer(&mut self) -> ParseResult<i64> {
        self.parse_positive_integer()
            .or_else(|_| self.parse_negative_integer())
    }

    pub fn parse_command(&mut self) -> ParseResult<Command> {
        self.parse_meta_command()
            .map(|cmd| Command::MetaCommand(cmd))
            .or_else(|e| {
                e.ignore_fail()?;
                self.parse_statement().map(|stmt| Command::Statement(stmt))
            })
    }

    fn parse_meta_command(&mut self) -> ParseResult<MetaCommand> {
        self.lex_string(".")?;
        self.lex_string("exit")
            .map(|_| MetaCommand::Exit)
            .or_else(|e| {
                e.ignore_fail()?;
                self.lex_string("print").map(|_| MetaCommand::Print)
            })
            .map_err(|_| ParseError::UnrecognizedMetaCommand)
    }

    fn parse_semicolon(&mut self) -> ParseResult<()> {
        self.lex_string(";")
            .map(|_| ())
            .map_err(|_| ParseError::MissingSemicolon)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let stmt = self
            .parse_select()
            .or_else(|e| {
                e.ignore_fail()?;
                self.parse_create_table()
            })
            .or_else(|e| {
                e.ignore_fail()?;
                self.parse_insert_into()
            })
            .or_else(|e| {
                e.ignore_fail()?;
                Err(ParseError::UnrecognizedStatement)
            })?;
        self.parse_semicolon()?;
        Ok(stmt)
    }

    fn parse_select(&mut self) -> ParseResult<Statement> {
        self.lex_string("select")?;
        let columns = self.parse_columns()?;
        self.lex_string("from")
            .map_err(|_| ParseError::MissingFrom)?;
        let table = self.lex_identifier()?;
        let condition = if let Ok(_) = self.lex_string("where") {
            Some(self.parse_condition()?)
        } else {
            None
        };
        Ok(Statement::Select {
            columns,
            table: String::from(table),
            condition,
        })
    }

    fn parse_left_paren(&mut self) -> ParseResult<()> {
        self.lex_string("(").map_err(|_| ParseError::MissingLParen)
    }

    fn parse_right_paren(&mut self) -> ParseResult<()> {
        self.lex_string(")").map_err(|_| {
            if let Ok(_) = self.lex_identifier() {
                ParseError::MissingComma
            } else {
                ParseError::MissingRParen
            }
        })
    }

    fn parse_columns(&mut self) -> ParseResult<Vec<Identifier>> {
        self.parse_left_paren()?;
        let ident = self.lex_identifier()?;
        let mut columns = vec![ident];
        while let Ok(_) = self.lex_string(",") {
            let ident = self.lex_identifier()?;
            columns.push(ident);
        }
        self.parse_right_paren()?;
        Ok(columns)
    }

    fn parse_column_pairs(&mut self) -> ParseResult<Vec<(Identifier, DBType)>> {
        self.parse_left_paren()?;
        let ident = self.lex_identifier()?;
        let db_type = self.parse_db_type()?;
        let mut columns = vec![(ident, db_type)];
        while let Ok(_) = self.lex_string(",") {
            let ident = self.lex_identifier()?;
            let db_type = self.parse_db_type()?;
            columns.push((ident, db_type));
        }
        self.parse_right_paren()?;
        Ok(columns)
    }

    fn parse_db_type(&mut self) -> ParseResult<DBType> {
        self.lex_string("integer")
            .map(|_| DBType::Integer)
            .or_else(|_| self.lex_string("text").map(|_| DBType::Text))
            .map_err(|e| {
                if let ParseError::EndOfInput = e {
                    ParseError::MissingType
                } else {
                    ParseError::UnrecognizedType
                }
            })
    }

    fn parse_create_table(&mut self) -> ParseResult<Statement> {
        self.lex_string("create")?;
        self.lex_string("table")?;
        let table = self.lex_identifier()?;
        let columns = self.parse_column_pairs()?;
        Ok(Statement::CreateTable { table, columns })
    }

    fn lex_value(&mut self) -> ParseResult<DBValue> {
        self.skip_whitespace();
        self.parse_integer()
            .map(|int| DBValue::Integer(int))
            .or_else(|e| {
                e.ignore_fail()?;
                self.parse_text().map(|text| DBValue::Text(text))
            })
    }

    fn parse_values(&mut self) -> ParseResult<Vec<DBValue>> {
        self.parse_left_paren()?;
        let value = self.lex_value()?;
        let mut columns = vec![value];
        while let Ok(_) = self.lex_string(",") {
            let ident = self.lex_value()?;
            columns.push(ident);
        }
        self.parse_right_paren()?;
        Ok(columns)
    }

    fn parse_insert_into(&mut self) -> ParseResult<Statement> {
        self.lex_string("insert")?;
        self.lex_string("into")?;
        let table = self.lex_identifier()?;
        self.lex_string("values")?;
        let values = self.parse_values().map_err(|e| {
            if let ParseError::FailedToLex = e {
                ParseError::InvalidValue
            } else {
                e
            }
        })?;
        Ok(Statement::InsertInto { table, values })
    }

    fn parse_condition(&mut self) -> ParseResult<Condition> {
        todo!();
    }
}
