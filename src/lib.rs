pub mod column;
pub mod common;
pub mod table;
pub mod types;

use serde_derive::{Deserialize, Serialize};
use table::CreateTable;

//TODO: IMPLEMENTS OTHERS COMMANDS
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlCommandQuery {
    CreateTable(CreateTable),
    Insert,
    Select,
    Delete,
    DropTable,
    Update,
    Set,
}
