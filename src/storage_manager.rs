use crate::db::*;
use crate::parser::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct StorageManager {
    tables: HashMap<String, Table>,
}

pub enum StorageError {
    TableNotFound,
    SchemaMismatch,
    TypeError,
    TableNameAlreadyInUse,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TableNotFound => write!(f, "Table not found"),
            Self::SchemaMismatch => write!(f, "Schema mismatch"),
            Self::TypeError => write!(f, "Type error"),
            Self::TableNameAlreadyInUse => write!(f, "Table name already in use"),
        }
    }
}

impl StorageManager {
    pub fn new() -> Self {
        StorageManager {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: String, schema: Schema) -> Result<(), StorageError> {
        if self.tables.contains_key(&name) {
            return Err(StorageError::TableNameAlreadyInUse);
        }
        self.tables.insert(name, Table::new(schema));
        Ok(())
    }

    pub fn insert_into(&mut self, table: String, values: Vec<DBValue>) -> Result<(), StorageError> {
        let table = self
            .tables
            .get_mut(&table)
            .ok_or(StorageError::TableNotFound)?;
        let types = values.iter().map(|val| val.val_to_type()).collect();
        table
            .schema()
            .type_check(types)
            .ok_or(StorageError::TypeError)?;
        table.push(values);
        Ok(())
    }

    // TODO: Refactor into relational set operators and expect that as a parameter
    // also note the schema/table interface
    pub fn query(&self, query: Statement) -> Result<Vec<Row>, StorageError> {
        if let Statement::Select { columns, table, .. } = query {
            let table = self.tables.get(&table).ok_or(StorageError::TableNotFound)?;
            let indices = table
                .schema()
                .get_column_indices(columns)
                .ok_or(StorageError::SchemaMismatch)?;
            let mut view = Vec::new();
            for row in table.rows() {
                let mut row_view = Vec::new();
                for i in &indices {
                    row_view.push(row[*i].clone());
                }
                view.push(row_view);
            }
            Ok(view)
        } else {
            Ok(Vec::new())
        }
    }
}
