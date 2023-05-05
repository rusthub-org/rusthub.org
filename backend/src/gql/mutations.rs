use async_graphql::Context;
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;

use crate::users::{
    self,
    models::{User, UserNew, Wish, WishNew},
};
use crate::creations::{
    self,
    models::{
        Creation, CreationNew, File, FileNew, CreationFile, CreationFileNew,
    },
};
use crate::topics::{
    self,
    models::{
        Topic, TopicNew, TopicUser, TopicUserNew, TopicCreation,
        TopicCreationNew,
    },
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_register(db, user_new).await
    }

    // Change user password
    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        pwd_cur: String,
        pwd_new: String,
        token: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_change_password(db, pwd_cur, pwd_new, token).await
    }

    // update user profile
    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
        token: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_update_profile(db, user_new, token).await
    }

    // modify user's one field by its id
    async fn user_update_one_field_by_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        field_name: String,
        field_val: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_update_one_field_by_id(
            db, user_id, field_name, field_val,
        )
        .await
    }

    // Add new creation
    async fn creation_new(
        &self,
        ctx: &Context<'_>,
        creation_new: CreationNew,
    ) -> GqlResult<Creation> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_new(db, creation_new).await
    }

    // modify creation's one field by its id
    async fn creation_update_one_field_by_id(
        &self,
        ctx: &Context<'_>,
        creation_id: ObjectId,
        field_name: String,
        field_val: String,
    ) -> GqlResult<Creation> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_update_one_field_by_id(
            db,
            creation_id,
            field_name,
            field_val,
        )
        .await
    }

    // Add new file
    async fn file_new(
        &self,
        ctx: &Context<'_>,
        file_new: FileNew,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::file_new(db, file_new).await
    }

    // Add new creation_file
    async fn creation_file_new(
        &self,
        ctx: &Context<'_>,
        creation_file_new: CreationFileNew,
    ) -> GqlResult<CreationFile> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations::services::creation_file_new(db, creation_file_new).await
    }

    // Add new topic
    async fn topic_new(
        &self,
        ctx: &Context<'_>,
        topic_new: TopicNew,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_new(db, topic_new).await
    }

    // Add new topics
    async fn topics_new(
        &self,
        ctx: &Context<'_>,
        topic_names: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_new(db, topic_names).await
    }

    // Add new topic_user
    async fn topic_user_new(
        &self,
        ctx: &Context<'_>,
        topic_user_new: TopicUserNew,
    ) -> GqlResult<TopicUser> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_user_new(db, topic_user_new).await
    }

    // Add new topic_creation
    async fn topic_creation_new(
        &self,
        ctx: &Context<'_>,
        topic_creation_new: TopicCreationNew,
    ) -> GqlResult<TopicCreation> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_creation_new(db, topic_creation_new).await
    }

    // Add new wish
    async fn wish_new(
        &self,
        ctx: &Context<'_>,
        wish_new: WishNew,
    ) -> GqlResult<Wish> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wish_new(db, wish_new).await
    }
}
