//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "video")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(unique)]
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub watch_num: Option<i32>,
    pub comment_num: Option<i32>,
    pub mylist_num: Option<i32>,
    pub category: Option<String>,
    pub length: Option<i32>,
    pub file_type: Option<String>,
    pub upload_time: Option<DateTimeWithTimeZone>,
    pub size_high: Option<i32>,
    pub size_low: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::video_tag_relation::Entity")]
    VideoTagRelation,
}

impl Related<super::video_tag_relation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VideoTagRelation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}