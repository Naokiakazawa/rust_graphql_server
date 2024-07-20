use crate::queries::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
