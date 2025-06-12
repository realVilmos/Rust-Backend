use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, PartialEq, Eq, Clone, Copy)]
#[DieselType = "Token_type"]  // must match the SQL type name
pub enum TokenType {
    #[db_rename = "access"]
    Access,
    #[db_rename = "refresh"]
    Refresh,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = active_tokens)]
pub struct ActiveToken {
    pub token: String,
    pub type_: TokenType,
    pub exp: chrono::NaiveDateTime,
}