use std::{path::PathBuf, fs::read_to_string, borrow::Cow};
use tide::Request;
use fluent_bundle::{FluentBundle, FluentResource, FluentArgs, FluentValue};
use serde_json::{Map, Value};

use crate::State;
use crate::util::constant::CFG;

use crate::models::users::SignStatus;

pub async fn gql_uri() -> String {
    let gql_prot = CFG.get("GQL_PROT").unwrap();
    let gql_addr = CFG.get("GQL_ADDR").unwrap();
    let gql_port = CFG.get("GQL_PORT").unwrap();
    let gql_uri = CFG.get("GQL_URI").unwrap();
    let gql_path = CFG.get("GQL_VER").unwrap();

    format!("{}://{}:{}/{}/{}", gql_prot, gql_addr, gql_port, gql_uri, gql_path)
}

pub async fn scripts_dir() -> String {
    format!("../assets/{}/", "scripts")
}

pub async fn tpls_dir() -> String {
    format!("./{}/", "templates")
}

pub async fn sign_status(req: &Request<State>) -> SignStatus {
    let username = if let Some(cookie) = req.cookie("username") {
        String::from(cookie.value())
    } else {
        String::from("-")
    };

    let token = if let Some(cookie) = req.cookie("token") {
        String::from(cookie.value())
    } else {
        String::from("-")
    };

    let sign_in = if let "" | "-" = username.trim() { false } else { true };

    SignStatus { sign_in, username, token }
}

pub fn get_lang_msg(
    lang_id: &str,
    root_tpl: &str,
    msg_id: &str,
    msg_args: Option<&Map<String, Value>>,
) -> String {
    let mut bundle = FluentBundle::default();

    let lang_res = get_lang_res(root_tpl);
    for res_file in lang_res {
        let res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../assets/locales")
            .join(lang_id)
            .join(res_file);
        let res_source = read_to_string(&res_path).expect(&format!(
            "Failed to read res file: {}.",
            res_path.to_str().unwrap()
        ));

        let resource = FluentResource::try_new(res_source.to_owned())
            .expect(&format!("{}, could not parse a LANG string.", res_source));
        bundle
            .add_resource(resource)
            .expect("Failed to add LANG resources to the bundle.");
    }

    let mut args = FluentArgs::new();
    if let Some(msg_map) = msg_args {
        for arg_key in msg_map.keys() {
            let arg_val =
                msg_map.get(arg_key).unwrap().as_str().unwrap_or_default();
            args.set(arg_key, FluentValue::from(arg_val));
        }
    }

    // let msg = bundle
    //     .get_message(msg_id)
    //     .expect(format!("{} is not exists", msg_id).as_str());
    // let pattern =
    //     msg.value().expect(format!("{} must have a value", msg_id).as_str());

    // let mut errors = vec![];
    // let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);

    let value = if let Some(fmsg) = bundle.get_message(msg_id) {
        let pattern = fmsg.value().unwrap();

        let mut errors = vec![];
        bundle.format_pattern(&pattern, Some(&args), &mut errors)
    } else {
        println!("\n\n\nmsg_id: {} 未被翻译\n\n\n", msg_id);

        Cow::from(msg_id)
    };

    value.to_string()
}

fn get_lang_res(root_tpl: &str) -> Vec<&str> {
    let mut tpl_lang_res = vec!["common.lang"];
    if let Some(dir) = root_tpl.split_once("_") {
        match dir.0 {
            "users" => {
                tpl_lang_res.push("users.lang");
                tpl_lang_res.push("pagination.lang");
            }
            "creations" => {
                tpl_lang_res.push("creations.lang");
                tpl_lang_res.push("pagination.lang");
            }
            "admin" => {
                tpl_lang_res.push("admin.lang");
                tpl_lang_res.push("users.lang");
                tpl_lang_res.push("creations.lang");
                tpl_lang_res.push("pagination.lang");
            }
            _ => (),
        }
    } else {
        tpl_lang_res.push("home.lang");
    }

    tpl_lang_res
}
