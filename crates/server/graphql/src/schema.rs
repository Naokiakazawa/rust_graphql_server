use crate::mutations::MutationRoot;
use crate::queries::QueryRoot;
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
