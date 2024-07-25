use std::fs::OpenOptions;
use std::io::Write;

use async_graphql::*;
use graphql::mutations::MutationRoot;
use graphql::queries::QueryRoot;

fn main() {
    let schema: Schema<QueryRoot, MutationRoot, EmptySubscription> =
        Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let sdl: String = schema.sdl();

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("schema.graphql")
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(sdl.as_bytes()) {
                eprintln!("Failed to write: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
        }
    }
}
