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

    async fn post(&self, ctx: &Context<'_>, id: i32) -> Result<Option<models::PostModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<entities::posts::Model> = Post::find_by_id(id).one(db).await?;
        Ok(post.map(|p: entities::posts::Model| p.into()))
    }

    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<models::PostModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let posts = Post::find().all(db).await?;
        Ok(posts.into_iter().map(|p| p.into()).collect())
    }

    async fn posts_per_page(
        &self,
        ctx: &Context<'_>,
        page: u64,
        page_size: u64,
    ) -> Result<models::PaginatedPosts, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let paginator = Post::find().paginate(db, page_size);
        let num_pages: u64 = paginator.num_pages().await?;
        let posts: Vec<posts::Model> = paginator.fetch_page(page - 1).await?;
        Ok(models::PaginatedPosts {
            total_pages: num_pages,
            posts: posts.into_iter().map(|p| p.into()).collect(),
        })
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<models::UserModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let user: Option<entities::users::Model> = User::find_by_id(id).one(db).await?;
        Ok(user.map(|u: entities::users::Model| u.into()))
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<models::UserModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let users = User::find().all(db).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    async fn users_per_page(
        &self,
        ctx: &Context<'_>,
        page: u64,
        page_size: u64,
    ) -> Result<models::PaginatedUsers, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let paginator = User::find().paginate(db, page_size);
        let num_pages: u64 = paginator.num_pages().await?;
        let users: Vec<users::Model> = paginator.fetch_page(page - 1).await?;
        Ok(models::PaginatedUsers {
            total_pages: num_pages,
            users: users.into_iter().map(|u| u.into()).collect(),
        })
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

    async fn comments(&self, ctx: &Context<'_>) -> Result<Vec<models::CommentModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let comments = Comment::find().all(db).await?;
        Ok(comments.into_iter().map(|c| c.into()).collect())
    }

    async fn comments_per_page(
        &self,
        ctx: &Context<'_>,
        page: u64,
        page_size: u64,
    ) -> Result<models::PaginatedComments, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let paginator = Comment::find().paginate(db, page_size);
        let num_pages: u64 = paginator.num_pages().await?;
        let comments: Vec<comments::Model> = paginator.fetch_page(page - 1).await?;
        Ok(models::PaginatedComments {
            total_pages: num_pages,
            comments: comments.into_iter().map(|c| c.into()).collect(),
        })
    }

    async fn comments_by_post_id(&self, ctx: &Context<'_>, post_id: i32) -> Result<Vec<models::CommentModel>, DbErr> {
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let post: Option<posts::Model> = Post::find_by_id(post_id).one(db).await?;
        let post: posts::Model = post.unwrap();
        let comments: Vec<comments::Model> = post.find_related(Comment).all(db).await?;
        Ok(comments.into_iter().map(|c| c.into()).collect())
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
