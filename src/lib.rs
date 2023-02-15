pub mod column;
pub mod common;
pub mod query;
pub mod table;
pub mod types;

use query::{create::CreateTable, insert::InsertTable};
use serde::{Deserialize, Serialize};

//TODO: IMPLEMENTS OTHERS COMMANDS
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlCommandQuery {
    CreateTable(CreateTable),
    Insert(InsertTable),
    Select,
    Delete,
    DropTable,
    Update,
    Set,
}
