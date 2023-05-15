use async_graphql::Context;
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::{
    constant::GqlResult,
    pagination::{UsersResult, CreationsResult},
};

use crate::users::{
    self,
    models::{User, SignInfo, Wish},
};
use crate::creations::{self, models::Creation};
use crate::topics::{self, models::Topic};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // user sign in
    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        signature: String,
        password: String,
    ) -> GqlResult<SignInfo> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_sign_in(db, signature, password).await
    }

    // get user info by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_username(db, username).await
    }

    // Get all Users
    async fn users(
        &self,
        ctx: &Context<'_>,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<UsersResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::users(db, from_page, first_oid, last_oid, status).await
    }

    // Get creation by its slug
    async fn creation_by_slug(
        &self,
        ctx: &Context<'_>,
        creation_slug: String,
    ) -> GqlResult<Creation> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_by_slug(db, creation_slug).await
    }

    // get random creation
    async fn creation_random_id(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<ObjectId> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_random_id(db).await
    }

    // Get all creations
    async fn creations(
        &self,
        ctx: &Context<'_>,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations(
            db, from_page, first_oid, last_oid, status,
        )
        .await
    }

    async fn creations_in_position(
        &self,
        ctx: &Context<'_>,
        username: String,
        position: String,
        limit: i64,
    ) -> GqlResult<Vec<Creation>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_in_position(
            db, username, position, limit,
        )
        .await
    }

    // Get all creations of one user by username
    async fn creations_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_by_username(
            db, username, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all creations by topic_slug
    async fn creations_by_topic_slug(
        &self,
        ctx: &Context<'_>,
        topic_slug: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_by_topic_slug(
            db, topic_slug, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all creations by filter
    async fn creations_by_filter(
        &self,
        ctx: &Context<'_>,
        filter_str: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_by_filter(
            db, filter_str, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // get topic info by slug
    async fn topic_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_by_slug(db, slug).await
    }

    // get all wishes
    async fn wishes(
        &self,
        ctx: &Context<'_>,
        status: i8,
    ) -> GqlResult<Vec<Wish>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wishes(db, status).await
    }

    // get random wish
    async fn wish_random(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Wish> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wish_random(db, username).await
    }
}
