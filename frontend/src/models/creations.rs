use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreationInfo {
    pub user_id: String,
    pub subject: String,
    pub cover_image_id: String,
    pub topic_names: String,
    pub content: String,
    pub website: String,
    pub source_url: String,
    pub res_file_ids: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    pub language: String,
}

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationsData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationsByUserData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationsByTopicData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationsByFilterData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationUpdateOneFieldByIdData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationRandomData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct FileNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/creations.graphql"
)]
pub struct CreationFileNewData;
