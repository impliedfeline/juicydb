use std::rc::Rc;
use std::collections::BTreeMap;
// Conceptually, a database is a collection of tables, a table is a collection of rows and a row is
// a collection of supported values with some means of indexing the values based on the column
// identifier

pub struct Database {
    name: String,
    tables: Vec<(String, Table)>,
}

impl Database {
    pub fn get_table(&mut self, id: &str) -> Option<&mut Table> {
        for (name, table) in &mut self.tables {
            if name == id { return Some(table) } 
        }
        None
    }
}

// TODO: support indexing with B-trees
pub struct Table {
    schema: Vec<(String, DBType)>,
    rows: Vec<Rc<Row>>,
}

impl Table {
    pub fn get_column_type(&self, id: &str) -> Option<DBType> {
        for (name, db_type) in &self.schema {
            if name == id { return Some(*db_type) } 
        }
        None
    }

    pub fn rows(&mut self) -> &mut Vec<Rc<Row>> {
        &mut self.rows
    }
}

pub struct Index(BTreeMap<DBValue, Rc<Row>>);

pub struct Row {
    values: Vec<(String, DBValue)>,
}

impl Row {
    pub fn get(&mut self, column: &str) -> Option<&mut DBValue> {
        for (name, value) in &mut self.values {
            if name == column { return Some(value) } 
        }
        None
    }
}

#[derive(Clone,Copy,Debug)]
pub enum DBType {
    Integer,
    Text,
}

#[derive(Clone,Debug)]
pub enum DBValue {
    Integer(i64),
    Text(String),
}

