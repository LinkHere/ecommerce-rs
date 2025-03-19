use sea_orm::entity::prelude::*;
use sea_orm_migration::seaql_migrations::Relation;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

//#[derive(Copy, Clone, Debug, EnumIter)]
//pub enum Relation {}

//impl Related<Relation> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
