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

    async fn post(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<models::PostModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<entities::posts::Model> = Post::find_by_id(id).one(db).await?;
        Ok(post.map(|p: entities::posts::Model| p.into()))
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<models::UserModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let user: Option<entities::users::Model> = User::find_by_id(id).one(db).await?;
        Ok(user.map(|u: entities::users::Model| u.into()))
    }

    async fn comment(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<models::CommentModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let comment: Option<entities::comments::Model> = Comment::find_by_id(id).one(db).await?;
        Ok(comment.map(|c: entities::comments::Model| c.into()))
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

impl From<users::Model> for models::UserModel {
    fn from(model: users::Model) -> Self {
        models::UserModel {
            id: model.id,
            username: model.username,
            email: model.email,
        }
    }
}

impl From<comments::Model> for models::CommentModel {
    fn from(model: comments::Model) -> Self {
        models::CommentModel {
            id: model.id,
            content: model.content,
            post_id: model.post_id,
            user_id: model.user_id,
        }
    }
}
