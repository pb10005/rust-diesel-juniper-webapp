use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

pub use crate::graphql::*;

graphql_schema_from_file!("src/graphql/schema/query.graphql");

pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<Vec<User>> {
        use crate::schema::users;

        users::table
            .load::<crate::models::User>(&executor.context().db_con)
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
            .load::<crate::models::Post>(&executor.context().db_con)
            .and_then(|posts| Ok(posts.into_iter().map_into().collect()))
            .map_err(Into::into)
    }

    fn field_user_by_id(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        id: i32,
    ) -> FieldResult<User> {
        use crate::schema::users::dsl::users;

        users.find(id)
            .first::<crate::models::User>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }

    fn field_post_by_id(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        id: i32,
    ) -> FieldResult<Post> {
        use crate::schema::posts::dsl::posts;

        posts.find(id)
            .first::<crate::models::Post>(&executor.context().db_con)
            .map(Into::into)
            .map_err(Into::into)
    }
}
