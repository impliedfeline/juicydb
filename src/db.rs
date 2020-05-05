use std::fmt;

/// Conceptually, a [`Database`] is a collection of [`Table`]s, a [`Table`] is a collection of
/// [`Row`]s and a [`Row`] is a collection of supported values with some means of indexing the
/// values based on the column identifier
pub struct Database {
    name: String,
    tables: Vec<(String, Table)>,
}

impl Database {
    pub fn get_table(&mut self, id: &str) -> Option<&mut Table> {
        for (name, table) in &mut self.tables {
            if name == id {
                return Some(table);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Table {
    schema: Schema,
    rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Schema {
    schema: Vec<(String, DBType)>,
}

impl Schema {
    pub fn new() -> Self {
        Self { schema: Vec::new() }
    }

    pub fn from(schema: Vec<(String, DBType)>) -> Self {
        Self { schema }
    }

    pub fn get_field_type(&self, id: &str) -> Option<DBType> {
        for (field, db_type) in &self.schema {
            if field == id {
                return Some(*db_type);
            }
        }
        None
    }

    pub fn get_column_indices(&self, columns: Vec<String>) -> Option<Vec<usize>> {
        let mut indices = Vec::new();
        for col in columns {
            let index = &self.schema.iter().position(|(f, _)| f == &col)?;
            indices.push(*index);
        }
        Some(indices)
    }

    pub fn type_check(&self, columns: Vec<DBType>) -> Option<()> {
        if columns.len() != self.schema.len() {
            return None;
        }

        for (t1, t2) in self.schema.iter().map(|(_, t)| t).zip(columns) {
            if *t1 != t2 {
                return None;
            }
        }
        Some(())
    }
}
pub type Row = Vec<DBValue>;

impl Table {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            rows: Vec::new(),
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Row> {
        &mut self.rows
    }

    pub fn push(&mut self, row: Row) {
        self.rows.push(row);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBType {
    Integer,
    Text,
}

impl fmt::Display for DBType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBType::Integer => write!(f, "integer"),
            DBType::Text => write!(f, "text"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DBValue {
    Integer(i64),
    Text(String),
}

impl DBValue {
    pub fn val_to_type(&self) -> DBType {
        match &self {
            DBValue::Integer(_) => DBType::Integer,
            DBValue::Text(_) => DBType::Text,
        }
    }
}

impl fmt::Display for DBValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBValue::Integer(i) => write!(f, "{}", i),
            DBValue::Text(text) => write!(f, "{}", text),
        }
    }
}
