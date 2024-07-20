use crate::models;
use async_graphql::{Context, Object, Result};
use entities::{post, post::Entity as Post};
use sea_orm::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
    async fn blog_post(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<models::PostModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<entities::post::Model> = Post::find_by_id(id).one(db).await?;
        Ok(post.map(|p: entities::post::Model| p.into()))
    }
}

impl From<post::Model> for models::PostModel {
    fn from(model: post::Model) -> Self {
        models::PostModel {
            id: model.id,
            title: model.title,
            content: model.text,
        }
    }
}
