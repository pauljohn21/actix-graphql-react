#[allow(unused_imports)]

use juniper::GraphQLInputObject;
use chrono::*;

use crate::schema::projects;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    pub website: String,
    pub source_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published: bool,
}

// applying #[derive(juniper::GraphQLObject)] to the project struct above
#[juniper::object(description = "A project of a user")]
impl Project {
    #[graphql(name = "id")]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "user_id")]
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    #[graphql(name = "subject")]
    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    #[graphql(name = "website")]
    pub fn website(&self) -> &str {
        self.website.as_str()
    }

    #[graphql(name = "source_code")]
    pub fn source_code(&self) -> &str {
        self.source_code.as_str()
    }

    #[graphql(name = "created_at")]
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    #[graphql(name = "updated_at")]
    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }

    #[graphql(name = "published")]
    pub fn published(&self) -> bool {
        self.published
    }
}

// Used to create new project
#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub user_id: &'a i32,
    pub subject: &'a str,
    pub website: &'a str,
    pub source_code: &'a str,
    pub published: &'a bool,

}

// The GraphQL input object for creating project
#[derive(GraphQLInputObject)]
pub struct ProjectInput {
    pub user_id: i32,
    pub subject: String,
    pub website: String,
    pub source_code: String,
    pub published: Option<bool>,
}