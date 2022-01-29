use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

pub use crate::graphql::*;

graphql_schema_from_file!("src/graphql/query.graphql");

impl QueryFields for Query {
    fn field_users(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<Vec<User>> {
        use crate::schema::users;

        users::table
            .load::<crate::models::users::User>(&executor.context().db_con)
            .and_then(|users| Ok(users.into_iter().map_into().collect()))
            .map_err(Into::into)
    }

    fn field_posts(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<Vec<Post>> {
        use crate::schema::posts;

        posts::table
            .load::<crate::models::posts::Post>(&executor.context().db_con)
            .and_then(|posts| Ok(posts.into_iter().map_into().collect()))
            .map_err(Into::into)
    }
}
