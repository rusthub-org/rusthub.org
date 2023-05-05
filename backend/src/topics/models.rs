use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::{constant::GqlResult, pagination::CreationsResult};

use crate::creations::services::creations_by_topic_id;

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Topic {
    pub _id: ObjectId,
    pub name: String,
    pub quotes: i64,
    pub slug: String,
}

#[async_graphql::ComplexObject]
impl Topic {
    pub async fn creations(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<CreationsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        creations_by_topic_id(
            db,
            self._id,
            1,
            String::from("-"),
            String::from("-"),
            1,
        )
        .await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct TopicNew {
    pub name: String,
    #[graphql(default = 1)]
    pub quotes: i64,
    #[graphql(skip)]
    pub slug: String,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct TopicUser {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub topic_id: ObjectId,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct TopicUserNew {
    pub user_id: ObjectId,
    pub topic_id: ObjectId,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct TopicCreation {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub creation_id: ObjectId,
    pub topic_id: ObjectId,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct TopicCreationNew {
    pub user_id: ObjectId,
    pub creation_id: ObjectId,
    pub topic_id: ObjectId,
}
