use crate::models;
use async_graphql::{Context, Object, Result};
use entities::{
    comments, comments::Entity as Comment, posts, posts::Entity as Post, users,
    users::Entity as User,
};
use sea_orm::*;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: models::CreatePostInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let new_post: posts::ActiveModel = posts::ActiveModel {
            title: Set(input.title),
            content: Set(input.content),
            user_id: Set(input.user_id),
            ..Default::default()
        };

        let res: sea_orm::InsertResult<posts::ActiveModel> =
            Post::insert(new_post).exec(db).await?;
        Ok(res.last_insert_id)
    }

    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        input: models::CreateCommentInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let new_comment: comments::ActiveModel = comments::ActiveModel {
            content: Set(input.content),
            post_id: Set(input.post_id),
            user_id: Set(input.user_id),
            ..Default::default()
        };
        let res: sea_orm::InsertResult<comments::ActiveModel> =
            Comment::insert(new_comment).exec(db).await?;
        Ok(res.last_insert_id)
    }

    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: models::CreateUserInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let new_user = users::ActiveModel {
            username: Set(input.username),
            email: Set(input.email),
            ..Default::default()
        };
        let res: InsertResult<users::ActiveModel> = User::insert(new_user).exec(db).await?;
        Ok(res.last_insert_id)
    }
}
