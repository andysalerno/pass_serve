use crate::simple_sql::simple_sql::{
    CreateTableResult, DeleteResult, InsertResult, QueryResult, SimpleSql,
};

struct SimpleSqlite;

impl SimpleSql for SimpleSqlite {
    fn CreateTable(&mut self) -> Result<CreateTableResult, &str> {}
    fn QueryTable(&self) -> QueryResult {}
    fn Insert(&self) -> InsertResult {}
    fn Delete(&mut self) -> DeleteResult {}
}
