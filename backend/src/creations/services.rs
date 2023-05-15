use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, DateTime, Document, doc, from_document, to_document,
        from_bson,
    },
    options::FindOptions,
};
use async_graphql::Error;
use chrono::Duration;

use crate::util::{
    {constant::GqlResult, common::slugify},
    common::{bdt_to_ymdhmsz, bdt_to_ymdhmsz_8},
    pagination::{
        CreationsResult, PageInfo, ResCount, count_pages_and_total,
        calculate_current_filter_skip, find_options,
    },
};
use crate::dbs::info::*;

use crate::users;
use crate::{topics, topics::models::TopicCreation};
use super::models::{
    Creation, CreationNew, File, FileNew, CreationFileNew, CreationFile,
};

const CREATIONS_STUFF: &str = "creations";

// create new creation
pub async fn creation_new(
    db: &Database,
    mut creation_new: CreationNew,
) -> GqlResult<Creation> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let now = DateTime::now();
    let now2ago = now.to_chrono() + Duration::days(-2);
    let filter_doc = doc! {
        "user_id": &creation_new.user_id,
        "subject": &creation_new.subject,
        "created_at": {"$gte": now2ago} // "$lte": now
    };
    let exist_document = coll.find_one(filter_doc, None).await?;

    if exist_document.is_none() {
        let slug = slugify(&creation_new.subject).await;
        creation_new.slug = format!("{}-{}", slug, now.timestamp_millis());

        let mut new_document = to_document(&creation_new)?;
        new_document.insert("created_at", now);
        new_document.insert("updated_at", now);

        let creation_res =
            coll.insert_one(new_document, None).await.expect(ERR_INSERT_PANIC);
        let creation_id = from_bson(creation_res.inserted_id)?;

        creation_by_id(db, creation_id).await
    } else {
        let creation: Creation = from_document(exist_document.unwrap())?;

        let created_time = format!(
            "{} / {} (CN)",
            bdt_to_ymdhmsz(creation.created_at).await,
            bdt_to_ymdhmsz_8(creation.created_at).await
        );
        Err(Error::new(created_time))
    }
}

pub async fn creation_by_slug(
    db: &Database,
    creation_slug: String,
) -> GqlResult<Creation> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let creation_document = coll
        .find_one(doc! {"slug": creation_slug}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let creation: Creation = from_document(creation_document)?;
    Ok(creation)
}

async fn creation_by_id(
    db: &Database,
    creation_id: ObjectId,
) -> GqlResult<Creation> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let creation_document = coll
        .find_one(doc! {"_id": creation_id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let creation: Creation = from_document(creation_document)?;
    Ok(creation)
}

pub async fn creation_update_one_field_by_slug(
    db: &Database,
    creation_slug: String,
    field_name: String,
    field_val: String,
) -> GqlResult<Creation> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let query_doc = doc! {"slug": creation_slug.to_owned()};
    let update_doc = match field_name.as_str() {
        "status" => {
            doc! {"$set": {
                field_name: field_val.parse::<i32>()?,
                "updated_at": DateTime::now()
            }}
        }
        "hits" | "insides" | "stars" => {
            doc! {"$inc": {field_name: field_val.parse::<i64>()?}}
        }
        _ => doc! {},
    };

    coll.update_one(query_doc, update_doc, None).await?;

    creation_by_slug(db, creation_slug).await
}

// get random creation
pub async fn creation_random_id(db: &Database) -> GqlResult<ObjectId> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let now = DateTime::now();
    let days_before = now.to_chrono() + Duration::days(-7);
    let filter_doc = doc! {
        "status": {"$gte": 1},
        "updated_at": {"$gte": days_before}
    };

    let match_doc = doc! {"$match": filter_doc};
    let mut cursor = coll
        .aggregate(vec![match_doc, doc! {"$sample": {"size": 1}}], None)
        .await?;

    if let Some(document_res) = cursor.next().await {
        let creation: Creation = from_document(document_res?)?;
        Ok(creation._id)
    } else {
        Err(Error::new(ERR_FIND_PANIC))
    }
}

pub async fn creations(
    db: &Database,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let mut filter_doc = doc! {};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut creations: Vec<Creation> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let creation = from_document(document)?;
                creations.push(creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let creations_result = CreationsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(CREATIONS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match creations.first() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            last_cursor: match creations.last() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: creations,
    };

    Ok(creations_result)
}

async fn filter_status(status: i8, filter_doc: &mut Document) {
    if status > 0 {
        filter_doc.insert("status", doc! {"$gte": status as i32});
    } else if status < 0 {
        filter_doc.insert("status", status as i32);
    }
}

pub async fn creations_in_position(
    db: &Database,
    username: String,
    position: String,
    limit: i64,
) -> GqlResult<Vec<Creation>> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let mut filter_doc = doc! {};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = users::services::user_by_username(db, username).await?;
        filter_doc.insert("user_id", &user._id);
    }

    match position.trim() {
        // "managed" => filter_doc.insert("status", doc! {"$gte": 6}),
        "recommended" => filter_doc.insert("status", doc! {"$gte": 2}),
        "published" => filter_doc.insert("status", doc! {"$gte": 1}),
        _ => None,
    };

    let sort_doc = doc! {"_id": -1};
    let find_options =
        FindOptions::builder().sort(sort_doc).limit(limit).build();
    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut creations: Vec<Creation> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let creation = from_document(document)?;
                creations.push(creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    Ok(creations)
}

pub async fn creations_by_username(
    db: &Database,
    username: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let user = users::services::user_by_username(db, username).await?;
    creations_by_user_id(db, user._id, from_page, first_oid, last_oid, status)
        .await
}

pub async fn creations_by_user_id(
    db: &Database,
    user_id: ObjectId,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let mut filter_doc = doc! {"user_id": user_id};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut creations: Vec<Creation> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let creation = from_document(document)?;
                creations.push(creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let creations_result = CreationsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(CREATIONS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match creations.first() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            last_cursor: match creations.last() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: creations,
    };

    Ok(creations_result)
}

// Get all creations by topic_slug
pub async fn creations_by_topic_slug(
    db: &Database,
    topic_slug: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let topic = topics::services::topic_by_slug(db, topic_slug).await?;
    creations_by_topic_id(db, topic._id, from_page, first_oid, last_oid, status)
        .await
}

// Get all creations by topic_id
pub async fn creations_by_topic_id(
    db: &Database,
    topic_id: ObjectId,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let topics_creations = topics_creations_by_topic_id(db, topic_id).await;

    let mut creation_ids = vec![];
    for topic_creation in topics_creations {
        creation_ids.push(topic_creation.creation_id);
    }
    creation_ids.sort();
    creation_ids.dedup();

    let coll = db.collection::<Document>(COLL_CREATIONS);

    let mut filter_doc = doc! {"_id": {"$in": creation_ids}};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut creations: Vec<Creation> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let creation = from_document(document)?;
                creations.push(creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let creations_result = CreationsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(CREATIONS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match creations.first() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            last_cursor: match creations.last() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: creations,
    };

    Ok(creations_result)
}

// get all TopicCreation list by topic_id
async fn topics_creations_by_topic_id(
    db: &Database,
    topic_id: ObjectId,
) -> Vec<TopicCreation> {
    let coll_topics_creations = db.collection::<Document>(COLL_TOPICS_RELEVANT);
    let mut cursor_topics_creations = coll_topics_creations
        .find(
            doc! {
                "topic_id": topic_id,
                "creation_id": { "$exists": true }
            },
            None,
        )
        .await
        .unwrap();

    let mut topics_creations: Vec<TopicCreation> = vec![];
    while let Some(result) = cursor_topics_creations.next().await {
        match result {
            Ok(document) => {
                let topic_creation: TopicCreation =
                    from_document(document).unwrap();
                topics_creations.push(topic_creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    topics_creations
}

// Get all creations by filter
pub async fn creations_by_filter(
    db: &Database,
    filter_str: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<CreationsResult> {
    let coll = db.collection::<Document>(COLL_CREATIONS);

    let mut filter_doc = doc! {};
    filter_status(status, &mut filter_doc).await;

    match filter_str.as_str() {
        "open-source" => {
            filter_doc.insert("source_url", doc! { "$ne": ""});
        }
        "with-website" => {
            filter_doc.insert("website", doc! { "$ne": ""});
        }
        _ => (),
    }

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut creations: Vec<Creation> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let creation = from_document(document)?;
                creations.push(creation);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let creations_result = CreationsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(CREATIONS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match creations.first() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            last_cursor: match creations.last() {
                Some(creation) => Some(creation._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: creations,
    };

    Ok(creations_result)
}

// Create new file
pub async fn file_new(db: &Database, file_new: FileNew) -> GqlResult<File> {
    let coll = db.collection::<Document>(COLL_FILES);

    let new_document = to_document(&file_new)?;

    let file_res =
        coll.insert_one(new_document, None).await.expect(ERR_INSERT_PANIC);
    let file_id = from_bson(file_res.inserted_id)?;

    file_by_id(db, file_id).await
}

// get file by id
async fn file_by_id(db: &Database, id: ObjectId) -> GqlResult<File> {
    let coll = db.collection::<Document>(COLL_FILES);

    let file_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let file: File = from_document(file_document)?;
    Ok(file)
}

// Create new creation_file
pub async fn creation_file_new(
    db: &Database,
    creation_file_new: CreationFileNew,
) -> GqlResult<CreationFile> {
    let coll = db.collection::<Document>(COLL_CREATIONS_FILES);

    let exist_document = coll
        .find_one(
            doc! {
            "user_id": &creation_file_new.user_id,
            "creation_id": &creation_file_new.creation_id,
            "file_id": &creation_file_new.file_id},
            None,
        )
        .await?;
    if exist_document.is_none() {
        let new_document = to_document(&creation_file_new)?;
        let creation_file_res =
            coll.insert_one(new_document, None).await.expect(ERR_INSERT_PANIC);
        let creation_file_id = from_bson(creation_file_res.inserted_id)?;

        creation_file_by_id(db, creation_file_id).await
    } else {
        Err(Error::new(ERR_RECORD_EXIST))
    }
}

// get creation_file by its id
async fn creation_file_by_id(
    db: &Database,
    id: ObjectId,
) -> GqlResult<CreationFile> {
    let coll = db.collection::<Document>(COLL_CREATIONS_FILES);

    let creation_file_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let creation_file: CreationFile = from_document(creation_file_document)?;
    Ok(creation_file)
}

// get file of one creation by file's kind & creation_id
pub async fn file_by_kind_creation_id(
    db: &Database,
    creation_id: ObjectId,
    file_kind: i8,
    file_status: i8,
) -> GqlResult<File> {
    let creations_files = creations_files_by_creation_id(db, creation_id).await;

    let mut file_ids = vec![];
    for creation_file in creations_files {
        file_ids.push(creation_file.file_id);
    }
    let filter_doc = doc! {
        "_id": {"$in": file_ids},
        "kind": file_kind as i32,
        "status": file_status as i32
    };

    let coll = db.collection::<Document>(COLL_FILES);
    let file_document = coll.find_one(filter_doc, None).await?;

    let file: File = if let Some(file_document) = file_document {
        from_document(file_document)?
    } else {
        let rusthub_cover = String::from("rusthub.png");
        File {
            _id: ObjectId::new(),
            name: rusthub_cover.to_owned(),
            kind: file_kind,
            location: rusthub_cover,
            status: 1,
        }
    };

    Ok(file)
}

// get all files of one creation by file's kind & creation_id
pub async fn files_by_kind_creation_id(
    db: &Database,
    creation_id: ObjectId,
    file_kind: i8,
    file_status: i8,
) -> GqlResult<Vec<File>> {
    let creations_files = creations_files_by_creation_id(db, creation_id).await;

    let mut file_ids = vec![];
    for creation_file in creations_files {
        file_ids.push(creation_file.file_id);
    }

    let filter_doc = doc! {
        "_id": {"$in": file_ids},
        "kind": file_kind as i32,
        "status": file_status as i32
    };

    let coll = db.collection::<Document>(COLL_FILES);
    let mut cursor = coll.find(filter_doc, None).await?;

    let mut files: Vec<File> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let file = from_document(document)?;
                files.push(file);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(files)
}

// get all CreationFile by creation_id
async fn creations_files_by_creation_id(
    db: &Database,
    creation_id: ObjectId,
) -> Vec<CreationFile> {
    let coll_creations_files = db.collection::<Document>(COLL_CREATIONS_FILES);
    let mut cursor_creations_files = coll_creations_files
        .find(doc! {"creation_id": creation_id}, None)
        .await
        .unwrap();

    let mut creations_files: Vec<CreationFile> = vec![];
    while let Some(result) = cursor_creations_files.next().await {
        match result {
            Ok(document) => {
                let creation_file: CreationFile =
                    from_document(document).unwrap();
                creations_files.push(creation_file);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    creations_files
}
