use std::collections::BTreeMap;
use tide::{Request, http::Method};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;

use crate::State;
use crate::util::{
    common::{gql_uri, sign_status},
    email::send_email,
    tpl::{Hbs, insert_user_by_username, insert_wish_random},
};

use crate::models::{
    Page,
    users::{
        UsersData, users_data, UserByUsernameData, user_by_username_data,
        UserUpdateOneFieldByUsernameData,
        user_update_one_field_by_username_data,
    },
};

pub async fn users_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut users_index_tpl: Hbs = Hbs::new("users/users-index").await;
    users_index_tpl
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
    users_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    data.insert("users-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let page: Page = req.query()?;
    let users_build_query = UsersData::build_query(users_data::Variables {
        from_page: page.from,
        first_oid: page.first,
        last_oid: page.last,
        status: 1,
    });
    let users_query = json!(users_build_query);

    let users_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(users_query).recv_json().await?;
    let users_resp_data = users_resp_body.data.unwrap();

    let users = users_resp_data["users"].clone();
    data.insert("pagination", users);

    users_index_tpl.render(&data).await
}

pub async fn user_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut user_index_tpl: Hbs = Hbs::new("users/users-user-detail").await;
    user_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        data.insert("sign-in", json!(sign_status.sign_in));
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let author_username = req.param("author_username")?;

    let user_update_hits_build_query =
        UserUpdateOneFieldByUsernameData::build_query(
            user_update_one_field_by_username_data::Variables {
                username: author_username.to_string(),
                field_name: String::from("hits"),
                field_val: String::from("1"),
            },
        );
    let user_update_hits_query = json!(user_update_hits_build_query);
    let _creation_update_hits_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(user_update_hits_query)
            .recv_json()
            .await?;

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
        author_by_username_resp_body.data.unwrap();

    let author_user = author_by_username_resp_data["userByUsername"].clone();
    data.insert("author_user", author_user);

    user_index_tpl.render(&data).await
}

pub async fn user_activate(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut user_activate_tpl: Hbs =
        Hbs::new("users/users-user-activate").await;
    user_activate_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_activate_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let author_username = req.param("author_username")?;
    match req.method() {
        Method::Post => {
            let user_resend_build_query = UserByUsernameData::build_query(
                user_by_username_data::Variables {
                    username: author_username.to_owned(),
                },
            );
            let user_resend_query = json!(user_resend_build_query);

            let user_resend_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(user_resend_query)
                    .recv_json()
                    .await?;
            let user_resend_resp_data = user_resend_resp_body.data.unwrap();

            let user_resend = user_resend_resp_data["userById"].clone();

            send_email(
                language,
                user_resend["username"].as_str().unwrap().to_string(),
                user_resend["nickname"].as_str().unwrap().to_string(),
                user_resend["email"].as_str().unwrap().to_string(),
            )
            .await;

            data.insert("user_resend", user_resend);
        }
        _ => {
            let user_activate_build_query =
                UserUpdateOneFieldByUsernameData::build_query(
                    user_update_one_field_by_username_data::Variables {
                        username: author_username.to_owned(),
                        field_name: String::from("activate"),
                        field_val: String::from("1"),
                    },
                );
            let user_activate_query = json!(user_activate_build_query);

            let user_activate_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(user_activate_query)
                    .recv_json()
                    .await?;
            let user_activate_resp_data = user_activate_resp_body.data.unwrap();

            let user_activate =
                user_activate_resp_data["userUpdateOneFieldByUsername"].clone();

            data.insert("user_activate", user_activate);
        }
    }

    user_activate_tpl.render(&data).await
}
