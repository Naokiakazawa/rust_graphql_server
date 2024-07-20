use async_graphql::SimpleObject;

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub content: String,
}
