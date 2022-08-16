//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub guild_id: String,
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub notifications: String,
    #[sea_orm(column_type = "Text")]
    pub color: String,
    pub is_all_day: bool,
    pub start_at: DateTime,
    pub end_at: DateTime,
    pub created_at: DateTime,
    pub discord_event_id: Option<String>,
    pub notify_channel_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}