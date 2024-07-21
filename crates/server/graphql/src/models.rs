use async_graphql::SimpleObject;

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct CommentModel {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
}
