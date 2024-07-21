use crate::models;
use async_graphql::{Context, Object, Result};
use entities::{
    comments, comments::Entity as Comment, posts, posts::Entity as Post, users,
    users::Entity as User,
};
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
        let post: Option<entities::posts::Model> = Post::find_by_id(id).one(db).await?;
        Ok(post.map(|p: entities::posts::Model| p.into()))
    }
}

impl From<posts::Model> for models::PostModel {
    fn from(model: posts::Model) -> Self {
        models::PostModel {
            id: model.id,
            title: model.title,
            content: model.content,
            user_id: model.user_id,
        }
    }
}
