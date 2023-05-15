use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, Document, doc, from_document, to_document, from_bson,
        DateTime,
    },
};
use async_graphql::Error;

use crate::util::{constant::GqlResult, common::slugify};
use crate::dbs::info::*;

use super::models::{
    Topic, TopicNew, TopicUser, TopicUserNew, TopicCreation, TopicCreationNew,
};

// get topic info by slug
pub async fn topic_by_slug(db: &Database, slug: String) -> GqlResult<Topic> {
    let coll = db.collection::<Document>(COLL_TOPICS);

    let topic_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let topic: Topic = from_document(topic_document)?;
    Ok(topic)
}

// Create new topics
pub async fn topics_new(
    db: &Database,
    topic_names: String,
) -> GqlResult<Vec<Topic>> {
    let mut topics: Vec<Topic> = vec![];

    let names = topic_names.split(",");
    for name in names {
        let topic_init = TopicNew {
            name: String::from(name.trim()),
            quotes: 1,
            slug: String::from(""),
        };

        let topic = topic_new(db, topic_init).await?;
        topics.push(topic);
    }

    Ok(topics)
}

// Create new topic_user
pub async fn topic_user_new(
    db: &Database,
    topic_user_new: TopicUserNew,
) -> GqlResult<TopicUser> {
    let coll = db.collection::<Document>(COLL_TOPICS_RELEVANT);

    let exist_document = coll
        .find_one(
            doc! {
                "topic_id": &topic_user_new.topic_id,
                "user_id": &topic_user_new.user_id,
                "creation_id": { "$exists": false }
            },
            None,
        )
        .await?;
    if exist_document.is_none() {
        let new_document = to_document(&topic_user_new)?;
        let topic_user_res =
            coll.insert_one(new_document, None).await.expect(ERR_INSERT_PANIC);
        let topic_user_id = from_bson(topic_user_res.inserted_id)?;

        topic_user_by_id(db, topic_user_id).await
    } else {
        Err(Error::new(ERR_RECORD_EXIST))
    }
}

// Create new topic_creation
pub async fn topic_creation_new(
    db: &Database,
    topic_creation_new: TopicCreationNew,
) -> GqlResult<TopicCreation> {
    let coll = db.collection::<Document>(COLL_TOPICS_RELEVANT);

    let exist_document = coll
        .find_one(
            doc! {
                "topic_id": &topic_creation_new.topic_id,
                "user_id": &topic_creation_new.user_id,
                "creation_id": &topic_creation_new.creation_id
            },
            None,
        )
        .await?;
    if exist_document.is_none() {
        let new_document = to_document(&topic_creation_new)?;
        let topic_creation_res =
            coll.insert_one(new_document, None).await.expect(ERR_INSERT_PANIC);
        let topic_creation_id = from_bson(topic_creation_res.inserted_id)?;

        topic_creation_by_id(db, topic_creation_id).await
    } else {
        Err(Error::new(ERR_RECORD_EXIST))
    }
}

// get users' keywords by user_id
pub async fn keywords_by_user_id(
    db: &Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_users = topics_users_by_user_id(db, user_id, false).await;

    let mut topic_ids = vec![];
    for topic_user in topics_users {
        topic_ids.push(topic_user.topic_id);
    }

    let coll = db.collection::<Document>(COLL_TOPICS);
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic: Topic = from_document(document)?;
                topics.push(topic);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// get topics by user_id
pub async fn topics_by_user_id(
    db: &Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_creations = topics_users_by_user_id(db, user_id, true).await;

    let mut topic_ids_dup = vec![];
    for topic_creation in topics_creations {
        topic_ids_dup.push(topic_creation.topic_id);
    }

    let mut topic_ids = topic_ids_dup.clone();
    topic_ids.sort();
    topic_ids.dedup();

    let mut topics: Vec<Topic> = vec![];
    let coll = db.collection::<Document>(COLL_TOPICS);
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let mut topic: Topic = from_document(document)?;
                topic.quotes =
                    topic_ids_dup.iter().filter(|&id| *id == topic._id).count()
                        as i64;
                topics.push(topic);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// get topics by creation_id
pub async fn topics_by_creation_id(
    db: &Database,
    creation_id: ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_creations =
        topics_creations_by_creation_id(db, creation_id).await;

    let mut topic_ids = vec![];
    for topic_creation in topics_creations {
        topic_ids.push(topic_creation.topic_id);
    }

    let coll = db.collection::<Document>(COLL_TOPICS);
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic = from_document(document)?;
                topics.push(topic);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// Create new topic
async fn topic_new(db: &Database, mut topic_new: TopicNew) -> GqlResult<Topic> {
    let coll = db.collection::<Document>(COLL_TOPICS);

    topic_new.name = topic_new.name.trim().to_lowercase();
    let name_check = "".ne(&topic_new.name) && "-".ne(&topic_new.name);
    match name_check {
        true => {
            let exist_document =
                coll.find_one(doc! {"name": &topic_new.name}, None).await?;

            let topic_id;
            if exist_document.is_none() {
                let slug = slugify(&topic_new.name).await;
                topic_new.slug =
                    format!("{}-{}", slug, DateTime::now().timestamp_millis());

                let new_document = to_document(&topic_new)?;
                let topic_res = coll
                    .insert_one(new_document, None)
                    .await
                    .expect(ERR_INSERT_PANIC);

                topic_id = from_bson(topic_res.inserted_id)?;
            } else {
                let topic: Topic = from_document(exist_document.unwrap())?;
                coll.update_one(
                    doc! {"_id": &topic._id},
                    doc! {"$inc": {"quotes": 1}},
                    None,
                )
                .await
                .expect(ERR_UPDATE_PANIC);

                topic_id = topic._id;
            }

            topic_by_id(db, topic_id).await
        }
        _ => Err(Error::new(ERR_NAME_ILLEGAL)),
    }
}

// get topic info by id
async fn topic_by_id(db: &Database, id: ObjectId) -> GqlResult<Topic> {
    let coll = db.collection::<Document>(COLL_TOPICS);

    let topic_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let topic: Topic = from_document(topic_document)?;
    Ok(topic)
}

// get topic_user by its id
async fn topic_user_by_id(db: &Database, id: ObjectId) -> GqlResult<TopicUser> {
    let coll = db.collection::<Document>(COLL_TOPICS_RELEVANT);

    let topic_user_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let topic_user: TopicUser = from_document(topic_user_document)?;
    Ok(topic_user)
}

// get topic_creation by its id
async fn topic_creation_by_id(
    db: &Database,
    id: ObjectId,
) -> GqlResult<TopicCreation> {
    let coll = db.collection::<Document>(COLL_TOPICS_RELEVANT);

    let topic_creation_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect(ERR_FIND_PANIC)
        .unwrap();

    let topic_creation: TopicCreation = from_document(topic_creation_document)?;
    Ok(topic_creation)
}

// get all TopicUser list by user_id
async fn topics_users_by_user_id(
    db: &Database,
    user_id: ObjectId,
    contain_creation: bool,
) -> Vec<TopicUser> {
    let coll_topics_users = db.collection::<Document>(COLL_TOPICS_RELEVANT);

    let mut filter_doc = doc! {"user_id": user_id};
    if !contain_creation {
        filter_doc.insert("creation_id", doc! { "$exists": contain_creation });
    }
    let mut cursor_topics_users =
        coll_topics_users.find(filter_doc, None).await.unwrap();

    let mut topics_users: Vec<TopicUser> = vec![];
    while let Some(result) = cursor_topics_users.next().await {
        match result {
            Ok(document) => {
                let topic_user: TopicUser = from_document(document).unwrap();
                topics_users.push(topic_user);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    topics_users
}

// get all TopicCreation list by creation_id
async fn topics_creations_by_creation_id(
    db: &Database,
    creation_id: ObjectId,
) -> Vec<TopicCreation> {
    let coll_topics_creations = db.collection::<Document>(COLL_TOPICS_RELEVANT);
    let mut cursor_topics_creations = coll_topics_creations
        .find(doc! {"creation_id": creation_id}, None)
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
