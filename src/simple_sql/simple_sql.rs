pub enum CreateTableStatus {
    TableCreated,
    TableExisted,
}

pub struct QueryResult;
pub struct InsertResult;
pub struct DeleteResult;

pub trait SimpleSql {
    fn CreateTable(&mut self) -> Result<CreateTableResult, &str>;
    fn QueryTable(&self) -> QueryResult;
    fn Insert(&self) -> InsertResult;
    fn Delete(&mut self) -> DeleteResult;
}
