#[derive(PartialEq, Eq, Clone)]
pub enum CommandType {
    Aggregate,
    Count,
    CreateCollection,
    CreateIndexes,
    CreateUser,
    DeleteMany,
    DeleteOne,
    Distinct,
    DropAllUsers,
    DropCollection,
    DropDatabase,
    DropIndexes,
    DropUser,
    Find,
    FindOneAndDelete,
    FindOneAndReplace,
    FindOneAndUpdate,
    GetUser,
    GetUsers,
    InsertMany,
    InsertOne,
    IsMaster,
    ListCollections,
    ListDatabases,
    ListIndexes,
    Suppressed,
    UpdateMany,
    UpdateOne,
}

impl CommandType {
    pub fn to_str(&self) -> &str {
        match self {
            &CommandType::Aggregate => "aggregate",
            &CommandType::Count => "count",
            &CommandType::CreateCollection => "create_collection",
            &CommandType::CreateIndexes => "create_indexes",
            &CommandType::CreateUser => "create_user",
            &CommandType::DeleteMany => "delete_many",
            &CommandType::DeleteOne => "delete_one",
            &CommandType::Distinct => "distinct",
            &CommandType::DropAllUsers => "drop_all_users",
            &CommandType::DropCollection => "drop_collection",
            &CommandType::DropDatabase => "drop_database",
            &CommandType::DropIndexes => "drop_indexes",
            &CommandType::DropUser => "drop_user",
            &CommandType::Find => "find",
            &CommandType::FindOneAndDelete => "find_one_and_delete",
            &CommandType::FindOneAndReplace => "find_one_and_replace",
            &CommandType::FindOneAndUpdate => "find_one_and_update",
            &CommandType::GetUser => "get_user",
            &CommandType::GetUsers => "get_users",
            &CommandType::InsertMany => "insert_many",
            &CommandType::InsertOne => "insert_one",
            &CommandType::IsMaster => "is_master",
            &CommandType::ListCollections => "list_collections",
            &CommandType::ListDatabases => "list_databases",
            &CommandType::ListIndexes => "list_indexes",
            &CommandType::Suppressed => "suppressed",
            &CommandType::UpdateMany => "update_many",
            &CommandType::UpdateOne => "update_one",
        }
    }
}