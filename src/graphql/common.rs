use crate::{DbCon};

pub struct Query;
pub struct Mutation;

pub struct Context {
    pub db_con: DbCon,
}
impl juniper::Context for Context {}

