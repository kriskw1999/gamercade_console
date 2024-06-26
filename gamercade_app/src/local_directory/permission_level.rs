use nohash_hasher::IsEnabled;
use rusqlite::types::FromSql;

use super::{Dictionary, DictionaryTrait};

#[derive(Eq, PartialEq, Debug, Default)]
pub struct PermissionLevelId(pub i32);

impl std::hash::Hash for PermissionLevelId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.0)
    }
}

impl IsEnabled for PermissionLevelId {}

#[derive(Default, Debug)]
pub struct PermissionLevel {
    pub name: String,
}

impl From<&rusqlite::Row<'_>> for PermissionLevel {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        Self {
            name: value.get_unwrap(1),
        }
    }
}

impl FromSql for PermissionLevelId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as i32))
    }
}

impl DictionaryTrait<PermissionLevelId, PermissionLevel>
    for Dictionary<PermissionLevelId, PermissionLevel>
{
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM permission_levels;"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS permission_levels(id INTEGER PRIMARY KEY, level_name TEXT NOT NULL) STRICT;"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS permission_levels;"
    }

    fn insert_query() -> &'static str {
        "INSERT INTO permission_levels(id, level_name) VALUES (?, ?)"
    }

    fn insert_statement(
        statement: &mut rusqlite::Statement,
        (key, value): &(PermissionLevelId, PermissionLevel),
    ) {
        statement.execute((key.0, value.name.clone())).unwrap();
    }
}
