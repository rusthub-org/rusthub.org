use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;
use chrono::FixedOffset;

// async-graphql result type
pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

// datetime format
pub const DTF_YMD: &str = "%Y-%m-%d";
pub const DTF_YMDHMSZ: &str = "%Y-%m-%d %H:%M:%S%:::z";

lazy_static! {
    // TimeZone +08:00:00
    pub static ref TZ_FO_8: Option<FixedOffset> = FixedOffset::east_opt(8 * 3600);

    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDR",
            dotenv::var("ADDR").expect("Expected ADDR to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "LOG_LEVEL",
            dotenv::var("LOG_LEVEL").expect("Expected LOG_LEVEL to be set in env!"),
        );

        map.insert(
            "SITE_KID",
            dotenv::var("SITE_KID").expect("Expected SITE_KID to be set in env!"),
        );
        map.insert(
            "SITE_KEY",
            dotenv::var("SITE_KEY").expect("Expected SITE_KEY to be set in env!"),
        );
        map.insert(
            "CLAIM_EXP",
            dotenv::var("CLAIM_EXP").expect("Expected CLAIM_EXP to be set in env!"),
        );

        map.insert(
            "GQL_URI",
            dotenv::var("GQL_URI").expect("Expected GQL_URI to be set in env!"),
        );
        map.insert(
            "GQL_VER",
            dotenv::var("GQL_VER").expect("Expected GQL_VER to be set in env!"),
        );
        map.insert(
            "GIQL_VER",
            dotenv::var("GIQL_VER").expect("Expected GIQL_VER to be set in env!"),
        );

        map.insert(
            "MONGODB_URI",
            dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_NAME",
            dotenv::var("MONGODB_NAME").expect("Expected MONGODB_NAME to be set in env!"),
        );
        map.insert(
            "PAGE_SIZE",
            dotenv::var("PAGE_SIZE").expect("Expected PAGE_SIZE to be set in env!"),
        );

        map
    };
}
