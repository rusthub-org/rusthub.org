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
use crate::creations::{
    self,
    models::{Creation, File},
};
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

    // get user info by id
    async fn user_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_id(db, id).await
    }

    // get user info by email
    async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_email(db, email).await
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

    // Get creation by its id
    async fn creation_by_id(
        &self,
        ctx: &Context<'_>,
        creation_id: ObjectId,
    ) -> GqlResult<Creation> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_by_id(db, creation_id).await
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

    // Get all creations of one user by user_id
    async fn creations_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_by_user_id(
            db, user_id, from_page, first_oid, last_oid, status,
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

    // Get all creations by topic_id
    async fn creations_by_topic_id(
        &self,
        ctx: &Context<'_>,
        topic_id: ObjectId,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creations_by_topic_id(
            db, topic_id, from_page, first_oid, last_oid, status,
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

    // get file by id
    async fn file_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::file_by_id(db, id).await
    }

    // get file of one creation by file's kind & creation_id
    async fn file_by_kind_creation_id(
        &self,
        ctx: &Context<'_>,
        creation_id: ObjectId,
        file_kind: i8,
        file_status: i8,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::file_by_kind_creation_id(
            db,
            creation_id,
            file_kind,
            file_status,
        )
        .await
    }

    // get all files of one creation by file's kind & creation_id
    async fn files_by_kind_creation_id(
        &self,
        ctx: &Context<'_>,
        creation_id: ObjectId,
        file_kind: i8,
        file_status: i8,
    ) -> GqlResult<Vec<File>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::files_by_kind_creation_id(
            db,
            creation_id,
            file_kind,
            file_status,
        )
        .await
    }

    // get topic info by id
    async fn topic_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_by_id(db, id).await
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

    // get all topics
    async fn topics(&self, ctx: &Context<'_>) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics(db).await
    }

    // get topics by creation_id
    async fn topics_by_creation_id(
        &self,
        ctx: &Context<'_>,
        creation_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_creation_id(db, creation_id).await
    }

    // get users' keywords by user_id
    async fn keywords_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::keywords_by_user_id(db, user_id).await
    }

    // get users' keywords by username
    async fn keywords_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::keywords_by_username(db, username).await
    }

    // get topics by user_id
    async fn topics_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_user_id(db, user_id).await
    }

    // get topics by username
    async fn topics_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_username(db, username).await
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
