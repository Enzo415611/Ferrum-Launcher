use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct UserRole {
    pub name: String,
    pub color: Option<String>,
}

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    // offline profile name and id
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub uuid: String,
    // for online profile
    pub refresh_token: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}
