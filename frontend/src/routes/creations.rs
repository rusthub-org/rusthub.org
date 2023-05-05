use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use async_std::path::Path;

use tide::{Request, Response, Redirect, http::Method};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;
use percent_encoding::percent_decode;

use crate::State;
use crate::util::{
    common::{gql_uri, sign_status},
    tpl::{Hbs, insert_user_by_username, insert_wish_random},
    upload::file_copy,
};

use crate::models::{
    Page,
    users::{UserByUsernameData, user_by_username_data},
    creations::{
        CreationInfo, CreationsData, creations_data, CreationsByUserData,
        creations_by_user_data, CreationsByTopicData, creations_by_topic_data,
        CreationsByFilterData, creations_by_filter_data, CreationData,
        creation_data, CreationNewData, creation_new_data,
        CreationUpdateOneFieldByIdData, creation_update_one_field_by_id_data,
        CreationRandomData, creation_random_data, FileNewData, file_new_data,
        CreationFileNewData, creation_file_new_data,
    },
    topics::{
        TopicsNewData, topics_new_data, TopicCreationNewData,
        topic_creation_new_data, TopicBySlugData, topic_by_slug_data,
    },
};

pub async fn creations_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut creations_index_tpl: Hbs =
        Hbs::new("creations/creations-index").await;
    creations_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    creations_index_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-creations-selected", json!("is-selected"));
    data.insert("creations-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let page: Page = req.query()?;
    let creations_build_query =
        CreationsData::build_query(creations_data::Variables {
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let creations_query = json!(creations_build_query);

    let creations_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creations_query)
            .recv_json()
            .await
            .unwrap();
    let creations_resp_data = creations_resp_body.data.expect("无响应数据");

    let creations = creations_resp_data["creations"].clone();
    data.insert("pagination", creations);

    creations_index_tpl.render(&data).await
}

pub async fn creations_by_user(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut creations_by_user_tpl: Hbs =
        Hbs::new("creations/creations-index").await;
    creations_by_user_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    creations_by_user_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-creations-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let author_username = req.param("author_username")?;
    let author_by_username_build_query =
        UserByUsernameData::build_query(user_by_username_data::Variables {
            username: String::from(author_username),
        });
    let author_by_username_query = json!(author_by_username_build_query);

    let author_by_username_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(author_by_username_query)
            .recv_json()
            .await
            .unwrap();
    let author_by_username_resp_data =
        author_by_username_resp_body.data.expect("无响应数据");

    let author = &author_by_username_resp_data["userByUsername"];
    let author_content = author["nickname"].as_str().unwrap().to_string()
        + " ("
        + author["username"].as_str().unwrap()
        + ")";
    data.insert(
        "filter_desc",
        json!({
            "condition": "user",
            "content": author_content
        }),
    );

    let page: Page = req.query()?;
    let creations_by_user_build_query =
        CreationsByUserData::build_query(creations_by_user_data::Variables {
            username: String::from(author_username),
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let creations_by_user_query = json!(creations_by_user_build_query);

    let creations_by_user_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creations_by_user_query)
            .recv_json()
            .await?;
    let creations_by_user_resp_data =
        creations_by_user_resp_body.data.expect("无响应数据");

    let creations_by_user =
        creations_by_user_resp_data["creationsByUsername"].clone();
    data.insert("pagination", creations_by_user);

    creations_by_user_tpl.render(&data).await
}

pub async fn creations_by_topic(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut creations_by_topic_tpl: Hbs =
        Hbs::new("creations/creations-index").await;
    creations_by_topic_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    creations_by_topic_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-creations-selected", json!("is-selected"));
    data.insert("creations-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let topic_slug = req.param("topic_slug")?;
    let topic_by_slug_build_query =
        TopicBySlugData::build_query(topic_by_slug_data::Variables {
            slug: String::from(topic_slug),
        });
    let topic_by_slug_query = json!(topic_by_slug_build_query);

    let topic_by_slug_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(topic_by_slug_query)
            .recv_json()
            .await
            .unwrap();
    let topic_by_slug_resp_data =
        topic_by_slug_resp_body.data.expect("无响应数据");

    let topic = &topic_by_slug_resp_data["topicBySlug"];
    data.insert(
        "filter_desc",
        json!({
            "condition": "topic",
            "content": topic["name"].as_str().unwrap()
        }),
    );

    let page: Page = req.query()?;
    let creations_by_topic_build_query =
        CreationsByTopicData::build_query(creations_by_topic_data::Variables {
            topic_slug: String::from(topic_slug),
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let creations_by_topic_query = json!(creations_by_topic_build_query);

    let creations_by_topic_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creations_by_topic_query)
            .recv_json()
            .await?;
    let creations_by_topic_resp_data =
        creations_by_topic_resp_body.data.expect("无响应数据");

    let creations_by_topic =
        creations_by_topic_resp_data["creationsByTopicSlug"].clone();
    data.insert("pagination", creations_by_topic);

    creations_by_topic_tpl.render(&data).await
}

pub async fn creations_filter(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut creations_filter_tpl: Hbs =
        Hbs::new("creations/creations-index").await;
    creations_filter_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    creations_filter_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-creations-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let filter_str = req.param("filter_str")?;
    let mut filter_status = 1;
    match filter_str {
        "recommended" => {
            data.insert("creations-recommended-selected", json!("is-selected"));
            filter_status = 2;
        }
        "open-source" => {
            data.insert("creations-open-source-selected", json!("is-selected"));
        }
        "with-website" => {
            data.insert(
                "creations-with-website-selected",
                json!("is-selected"),
            );
        }
        _ => (),
    }

    let page: Page = req.query()?;
    let creations_by_filter_build_query = CreationsByFilterData::build_query(
        creations_by_filter_data::Variables {
            filter_str: filter_str.to_string(),
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: filter_status,
        },
    );
    let creations_by_filter_query = json!(creations_by_filter_build_query);

    let creations_by_filter_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creations_by_filter_query)
            .recv_json()
            .await?;
    let creations_by_filter_resp_data =
        creations_by_filter_resp_body.data.expect("无响应数据");

    let creations_by_filter =
        creations_by_filter_resp_data["creationsByFilter"].clone();
    data.insert("pagination", creations_by_filter);

    let filter_desc = json!({
        "condition": filter_str,
        "content": format!("creations-filter-{}", filter_str)
    });
    data.insert("filter_desc", filter_desc);

    creations_filter_tpl.render(&data).await
}

pub async fn creation_new(mut req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut creation_new_tpl: Hbs =
            Hbs::new("creations/creations-creation-new").await;
        creation_new_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        creation_new_tpl.reg_script_values().await.reg_script_lang().await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-creations-selected", json!("is-selected"));
        insert_wish_random(&mut data).await;
        insert_user_by_username(sign_status.username, &mut data).await;

        match req.method() {
            Method::Post => {
                let creation_info: CreationInfo = req.body_form().await?;

                let creation_new_build_query = CreationNewData::build_query(
                    creation_new_data::Variables {
                        user_id: creation_info.user_id.clone(),
                        subject: creation_info.subject.clone(),
                        content: creation_info.content,
                        website: creation_info.website,
                        source_url: creation_info.source_url,
                        contact_user: creation_info.contact_user,
                        contact_phone: creation_info.contact_phone,
                        contact_email: creation_info.contact_email,
                        contact_im: creation_info.contact_im,
                        language: creation_info.language,
                    },
                );
                let creation_new_query = json!(creation_new_build_query);

                let creation_new_resp_body: GqlResponse<serde_json::Value> =
                    surf::post(&gql_uri().await)
                        .body(creation_new_query)
                        .recv_json()
                        .await?;
                let creation_new_resp_data = creation_new_resp_body.data;

                if let Some(creation_new_val) = creation_new_resp_data {
                    let creation_new_result =
                        creation_new_val["creationNew"].clone();
                    let creation_id =
                        creation_new_result["id"].as_str().unwrap();

                    // create topics
                    let topics_build_query = TopicsNewData::build_query(
                        topics_new_data::Variables {
                            topic_names: creation_info.topic_names,
                        },
                    );
                    let topics_query = json!(topics_build_query);

                    let topics_resp_body: GqlResponse<serde_json::Value> =
                        surf::post(&gql_uri().await)
                            .body(topics_query)
                            .recv_json()
                            .await?;
                    let topics_resp_data = topics_resp_body.data;

                    // create TopicCreation
                    if let Some(topics_info) = topics_resp_data {
                        let topic_ids =
                            topics_info["topicsNew"].as_array().unwrap();
                        for topic_id in topic_ids {
                            let topic_id = topic_id["id"].as_str().unwrap();
                            let topic_creation_new_build_query =
                                TopicCreationNewData::build_query(
                                    topic_creation_new_data::Variables {
                                        user_id: creation_info.user_id.clone(),
                                        creation_id: creation_id.to_string(),
                                        topic_id: topic_id.to_string(),
                                    },
                                );
                            let topic_creation_new_query =
                                json!(topic_creation_new_build_query);
                            let _topic_creation_new_resp_body: GqlResponse<
                                serde_json::Value,
                            > = surf::post(&gql_uri().await)
                                .body(topic_creation_new_query)
                                .recv_json()
                                .await?;
                        }
                    }

                    // create CreationFile
                    let file_ids = format!(
                        "{},{}",
                        creation_info.cover_image_id,
                        creation_info.res_file_ids
                    );
                    for file_id in file_ids.split(",") {
                        let creation_file_new_build_query =
                            CreationFileNewData::build_query(
                                creation_file_new_data::Variables {
                                    user_id: creation_info.user_id.clone(),
                                    creation_id: creation_id.to_string(),
                                    file_id: file_id.to_string(),
                                },
                            );
                        let creation_file_new_query =
                            json!(creation_file_new_build_query);
                        let _creation_file_new_resp_body: GqlResponse<
                            serde_json::Value,
                        > = surf::post(&gql_uri().await)
                            .body(creation_file_new_query)
                            .recv_json()
                            .await?;
                    }

                    data.insert("creation_new_result", creation_new_result);
                } else {
                    data.insert(
                        "creation_new_failed",
                        json!({
                            "subject": creation_info.subject,
                            "create_at": creation_new_resp_body.errors.unwrap()[0].message
                        })
                    );
                }
            }
            _ => (),
        }

        creation_new_tpl.render(&data).await
    } else {
        let resp: Response =
            Redirect::new(format!("/{}/sign-in", language)).into();

        Ok(resp.into())
    }
}

pub async fn creation_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut creation_index_tpl: Hbs =
        Hbs::new("creations/creations-creation-detail").await;
    creation_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    creation_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-creations-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        data.insert("sign-in", json!(sign_status.sign_in));
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let creation_id = req.param("creation_id")?;

    let creation_update_hits_build_query =
        CreationUpdateOneFieldByIdData::build_query(
            creation_update_one_field_by_id_data::Variables {
                creation_id: creation_id.to_string(),
                field_name: String::from("hits"),
                field_val: String::from("3"),
            },
        );
    let creation_update_hits_query = json!(creation_update_hits_build_query);
    let _creation_update_hits_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creation_update_hits_query)
            .recv_json()
            .await?;

    let creation_build_query =
        CreationData::build_query(creation_data::Variables {
            creation_id: creation_id.to_string(),
        });
    let creation_query = json!(creation_build_query);

    let creation_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(creation_query).recv_json().await?;
    let creation_resp_data = creation_resp_body.data.expect("无响应数据");

    let creation = creation_resp_data["creationById"].clone();
    data.insert("creation", creation);

    creation_index_tpl.render(&data).await
}

pub async fn creation_random(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let creation_random_build_query =
        CreationRandomData::build_query(creation_random_data::Variables {});
    let creation_random_query = json!(creation_random_build_query);

    let creation_random_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(creation_random_query)
            .recv_json()
            .await?;
    let creation_random_resp_data =
        creation_random_resp_body.data.expect("无响应数据");

    let creation_random_id =
        creation_random_resp_data["creationRandomId"].as_str().unwrap();
    let resp: Response =
        Redirect::new(format!("/{}/creation/{}", language, creation_random_id))
            .into();

    Ok(resp.into())
}

pub async fn file_new(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let file_name_percent = req.param("file_name")?;
    let file_name_percent_de = percent_decode(file_name_percent.as_bytes());
    let file_name = String::from(file_name_percent_de.decode_utf8()?);

    let file_kind = req.param("file_kind")?.parse::<i64>()?;

    let now_micros = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros();

    let file_ext_index = file_name.rfind(".").unwrap();
    let file_ext = &file_name[file_ext_index..];

    let mut file_location = String::new();
    file_location.push_str(now_micros.to_string().as_str());
    file_location.push_str(file_ext);

    let file_path_dir = String::from("../files/creations");
    let file_path = Path::new(&file_path_dir).join(&file_location);

    let file_copy = file_copy(req, file_path).await;

    let res;
    if file_copy.is_ok() {
        let file_new_build_query =
            FileNewData::build_query(file_new_data::Variables {
                name: file_name.clone(),
                kind: file_kind,
                location: file_location,
            });
        let file_new_query = json!(file_new_build_query);

        let file_new_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(file_new_query)
                .recv_json()
                .await?;
        let file_new_resp_data = file_new_resp_body.data.expect("无响应数据");

        let file_new_result = &file_new_resp_data["fileNew"];
        let file_id = file_new_result["id"].as_str().unwrap();

        res = json!({
            "done": true,
            "file_id": file_id,
            "file_name": file_name,
        });
    } else {
        println!("\n\n\n文件上传失败：{:?} \n\n\n", file_copy.err().unwrap());

        let err = match language.as_str() {
            "zh-cn" => "上传异常：请联系",
            _ => "Upload exception: please contact",
        };

        res = json!({
            "done": false,
            "err": format!("{} {}", err, "ask@rusthub.org")
        });
    }

    Ok(res.into())
}
