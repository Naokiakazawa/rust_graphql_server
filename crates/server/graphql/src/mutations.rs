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

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        input: models::UpdatePostInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<posts::Model> = Post::find_by_id(input.id).one(db).await?;
        let mut post: posts::ActiveModel = post.unwrap().into();

        if let Some(title) = input.title {
            post.title = Set(title);
        }
        if let Some(content) = input.content {
            post.content = Set(content);
        }

        let res: posts::Model = post.update(db).await?;
        Ok(res.id)
    }

    async fn delete_post(
        &self,
        ctx: &Context<'_>,
        input: models::DeletePostInput,
    ) -> Result<u64, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<posts::Model> = Post::find_by_id(input.id).one(db).await?;
        let post: posts::Model = post.unwrap();
        let res: DeleteResult = post.delete(db).await?;
        Ok(res.rows_affected)
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

    async fn update_comment(
        &self,
        ctx: &Context<'_>,
        input: models::UpdateCommentInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let comment: Option<comments::Model> = Comment::find_by_id(input.id).one(db).await?;
        let mut comment: comments::ActiveModel = comment.unwrap().into();
        if let Some(content) = input.content {
            comment.content = Set(content);
        }
        let res: comments::Model = comment.update(db).await?;
        Ok(res.id)
    }

    async fn delete_comment(
        &self,
        ctx: &Context<'_>,
        input: models::DeleteCommentInput,
    ) -> Result<u64, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let comment: Option<comments::Model> = Comment::find_by_id(input.id).one(db).await?;
        let comment: comments::Model = comment.unwrap();
        let res: DeleteResult = comment.delete(db).await?;
        Ok(res.rows_affected)
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

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: models::UpdateUserInput,
    ) -> Result<i32, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let user: Option<users::Model> = User::find_by_id(input.id).one(db).await?;
        let mut user: users::ActiveModel = user.unwrap().into();
        if let Some(username) = input.username {
            user.username = Set(username);
        }
        if let Some(email) = input.email {
            user.email = Set(email);
        }
        let res: users::Model = user.update(db).await?;
        Ok(res.id)
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        input: models::DeleteUserInput,
    ) -> Result<u64, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let user: Option<users::Model> = User::find_by_id(input.id).one(db).await?;
        let user: users::Model = user.unwrap();
        let res: DeleteResult = user.delete(db).await?;
        Ok(res.rows_affected)
    }
}
