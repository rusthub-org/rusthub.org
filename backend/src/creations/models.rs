use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

use crate::util::{
    constant::GqlResult,
    common::{bdt_to_ymd, bdt_to_ymd8, bdt_to_ymdhmsz, bdt_to_ymdhmsz_8},
};
use crate::dbs::mongo::DataSource;

use crate::{
    topics::{self, models::Topic},
    users::{self, models::User},
};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Creation {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub subject: String,
    pub slug: String,
    pub content: String,
    pub website: String,
    pub source_url: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub hits: u64,
    pub insides: u64,
    pub stars: u64,
    pub language: String,
    pub status: i8,
}

#[async_graphql::ComplexObject]
impl Creation {
    pub async fn cover_image(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        super::services::file_by_kind_creation_id(db, self._id, 1, 1).await
    }

    pub async fn res_files(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<File>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        super::services::files_by_kind_creation_id(db, self._id, 2, 1).await
    }

    pub async fn content_html(&self) -> String {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        // options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        // options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(&self.content, options);

        let mut content_html = String::new();
        html::push_html(&mut content_html, parser);

        content_html
    }

    pub async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_id(db, self.user_id).await
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_creation_id(db, self._id).await
    }

    pub async fn created_at_ymd(&self) -> String {
        bdt_to_ymd(self.created_at).await
    }

    pub async fn created_at_ymd8(&self) -> String {
        bdt_to_ymd8(self.created_at).await
    }

    pub async fn updated_at_ymd(&self) -> String {
        bdt_to_ymd(self.updated_at).await
    }

    pub async fn updated_at_ymd8(&self) -> String {
        bdt_to_ymd8(self.updated_at).await
    }

    pub async fn created_at_ymdhmsz(&self) -> String {
        bdt_to_ymdhmsz(self.created_at).await
    }

    pub async fn created_at_ymdhmsz_8(&self) -> String {
        bdt_to_ymdhmsz_8(self.created_at).await
    }

    pub async fn updated_at_ymdhmsz(&self) -> String {
        bdt_to_ymdhmsz(self.updated_at).await
    }

    pub async fn updated_at_ymdhmsz_8(&self) -> String {
        bdt_to_ymdhmsz_8(self.updated_at).await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct CreationNew {
    pub user_id: ObjectId,
    pub subject: String,
    #[graphql(skip)]
    pub slug: String,
    pub content: String,
    pub website: String,
    pub source_url: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    #[graphql(skip)]
    pub hits: u64,
    #[graphql(skip)]
    pub insides: u64,
    #[graphql(skip)]
    pub stars: u64,
    pub language: String,
    #[graphql(skip)]
    pub status: i8,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub _id: ObjectId,
    pub name: String,
    pub kind: i8,
    pub location: String,
    pub status: i8,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct FileNew {
    pub name: String,
    pub kind: i8,
    pub location: String,
    #[graphql(skip)]
    pub status: i8,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct CreationFile {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub creation_id: ObjectId,
    pub file_id: ObjectId,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct CreationFileNew {
    pub user_id: ObjectId,
    pub creation_id: ObjectId,
    pub file_id: ObjectId,
}
