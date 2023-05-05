use tide::{self, Server};

pub mod home;
pub mod users;
pub mod creations;
pub mod topics;
pub mod admin;

use crate::State;
use crate::util::common::tpls_dir;

pub async fn push_res(app: &mut Server<State>) {
    app.at("/").get(super::routes::home::init);

    app.at("/static/*").serve_dir("../assets/static/").unwrap();
    app.at("/files/*").serve_dir("../files/").unwrap();

    app.at("/ads.txt")
        .serve_file(format!("{}{}", tpls_dir().await, "ads.txt"))
        .unwrap_or_default();
    app.at("/sitemap.txt")
        .serve_file(format!("{}{}", tpls_dir().await, "sitemap.txt"))
        .unwrap_or_default();

    let mut admin = app.at("/admin");
    admin.at("/").get(super::routes::admin::admin_index);
    admin.at("/creations").get(super::routes::admin::creations_admin);
    admin
        .at("/creation/:creation_id")
        .get(super::routes::admin::creation_admin);
    admin
        .at("/creation/:creation_id/:field_name/:field_val")
        .get(super::routes::admin::creation_update_one_field);

    let mut home = app.at("/:language");
    home.at("/").get(super::routes::home::index);
    home.at("/register")
        .get(super::routes::home::register)
        .post(super::routes::home::register);
    home.at("/sign-in")
        .get(super::routes::home::sign_in)
        .post(super::routes::home::sign_in);
    home.at("/sign-out").get(super::routes::home::sign_out);

    let mut users = home.at("/users");
    users.at("/").get(super::routes::users::users_index);
    // users.at("/:filter_str").get(super::routes::users::users_filter);

    let mut user = home.at("/user");
    user.at("/:user_id/activate")
        .get(super::routes::users::user_activate)
        .post(super::routes::users::user_activate);
    user.at("/:author_username").get(super::routes::users::user_index);
    user.at("/:author_username/creations")
        .get(super::routes::creations::creations_by_user);

    let mut creations = home.at("/creations");
    creations.at("/").get(super::routes::creations::creations_index);
    creations
        .at("/:filter_str")
        .get(super::routes::creations::creations_filter);

    let mut creation = home.at("/creation");
    creation.at("/").get(super::routes::creations::creation_random);
    creation
        .at("/new")
        .get(super::routes::creations::creation_new)
        .post(super::routes::creations::creation_new);
    creation.at("/:creation_id").get(super::routes::creations::creation_index);
    creation
        .at("/file/new/:file_name/:file_kind")
        .put(super::routes::creations::file_new);

    // let mut topics = app.at("/topics");
    let mut topic = home.at("/topic");
    topic
        .at("/:topic_slug/creations")
        .get(super::routes::creations::creations_by_topic);
}
