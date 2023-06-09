use std::collections::BTreeMap;
use tide::{Request, Response, Redirect};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;

use crate::State;
use crate::util::{
    common::{gql_uri, sign_status, get_user_by_username},
    tpl::{Hbs, insert_user_by_username},
};

use crate::models::{
    Page,
    creations::{
        CreationsData, creations_data, CreationData, creation_data,
        CreationUpdateOneFieldBySlugData,
        creation_update_one_field_by_slug_data,
    },
};

pub async fn admin_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut admin_index_tpl: Hbs = Hbs::new("admin/admin-index").await;
        admin_index_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        admin_index_tpl.reg_script_values().await.reg_script_lang().await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        admin_index_tpl.render(&data).await
    } else {
        let resp: Response =
            Redirect::new(format!("/{}/sign-in", language)).into();

        Ok(resp.into())
    }
}

pub async fn creations_admin(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut admin_creations_tpl: Hbs =
            Hbs::new("admin/admin-creations").await;
        admin_creations_tpl
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
        admin_creations_tpl.reg_script_values().await.reg_script_lang().await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        let page: Page = req.query()?;
        let creations_build_query =
            CreationsData::build_query(creations_data::Variables {
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 0,
            });
        let creations_query = json!(creations_build_query);

        let creations_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(creations_query)
                .recv_json()
                .await
                .unwrap();
        let creations_resp_data = creations_resp_body.data.unwrap();

        let creations = creations_resp_data["creations"].clone();
        data.insert("pagination", creations);

        admin_creations_tpl.render(&data).await
    } else {
        let resp: Response =
            Redirect::new(format!("/{}/sign-in", language)).into();

        Ok(resp.into())
    }
}

pub async fn creation_admin(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut creation_index_tpl: Hbs =
            Hbs::new("admin/admin-creation-detail").await;
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
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        let creation_slug = req.param("creation_slug")?;

        let creation_update_hits_build_query =
            CreationUpdateOneFieldBySlugData::build_query(
                creation_update_one_field_by_slug_data::Variables {
                    creation_slug: creation_slug.to_string(),
                    field_name: String::from("hits"),
                    field_val: String::from("1"),
                },
            );
        let creation_update_hits_query =
            json!(creation_update_hits_build_query);
        let _creation_update_hits_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(creation_update_hits_query)
                .recv_json()
                .await?;

        let creation_build_query =
            CreationData::build_query(creation_data::Variables {
                creation_slug: creation_slug.to_owned(),
            });
        let creation_query = json!(creation_build_query);

        let creation_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(creation_query)
                .recv_json()
                .await?;
        let creation_resp_data = creation_resp_body.data.unwrap();

        let creation = creation_resp_data["creationBySlug"].clone();
        data.insert("creation", creation);

        creation_index_tpl.render(&data).await
    } else {
        let resp: Response =
            Redirect::new(format!("/{}/sign-in", language)).into();

        Ok(resp.into())
    }
}

pub async fn admin_creation_update_one_field(
    req: Request<State>,
) -> tide::Result {
    let language = String::from(req.param("language")?);

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let sign_user =
            get_user_by_username(sign_status.username.to_owned()).await;
        let sign_user_status = sign_user["status"].as_i64().unwrap();
        if sign_user_status >= 10 {
            let creation_slug = req.param("creation_slug")?;
            let field_name = req.param("field_name")?;
            let field_val = req.param("field_val")?;

            let creation_update_one_field_build_query =
                CreationUpdateOneFieldBySlugData::build_query(
                    creation_update_one_field_by_slug_data::Variables {
                        creation_slug: creation_slug.to_owned(),
                        field_name: String::from(field_name),
                        field_val: String::from(field_val),
                    },
                );
            let creation_update_one_field_query =
                json!(creation_update_one_field_build_query);
            let _creation_update_one_field_resp_body: GqlResponse<
                serde_json::Value,
            > = surf::post(&gql_uri().await)
                .body(creation_update_one_field_query)
                .recv_json()
                .await?;

            let resp: Response = Redirect::new(format!(
                "/{}/admin/creation/{}",
                language, creation_slug
            ))
            .into();

            Ok(resp.into())
        } else {
            let resp: Response = Redirect::new(format!(
                "/{}/user/{}",
                language, sign_status.username
            ))
            .into();

            Ok(resp.into())
        }
    } else {
        let resp: Response =
            Redirect::new(format!("/{}/sign-in", language)).into();

        Ok(resp.into())
    }
}
