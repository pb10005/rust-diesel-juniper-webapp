use std::convert::From;
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

pub use crate::graphql::context::*;

graphql_schema_from_file!("src/graphql/schema/user.graphql");

pub struct User {
    pub id: i32,
    pub name: String,
}

impl UserFields for User {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_string()))
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
}

impl From<crate::models::User> for User {
    fn from(user: crate::models::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}
