use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct UpdatePostInput {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct DeletePostInput {
    pub id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct UpdateUserInput {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct DeleteUserInput {
    pub id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct CommentModel {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct CreateCommentInput {
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct UpdateCommentInput {
    pub id: i32,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, InputObject)]
pub struct DeleteCommentInput {
    pub id: i32,
}
