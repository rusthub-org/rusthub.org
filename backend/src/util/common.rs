// Generate friendly slug from the given string
pub async fn slugify(str: &str) -> String {
    use deunicode::deunicode_with_tofu;
    use regex::Regex;

    let slug = deunicode_with_tofu(str.trim(), "-").to_lowercase();

    let re = Regex::new(r"[^a-z-0-9]").unwrap();
    let slug = re.replace_all(&slug, "-");

    let re = Regex::new(r"-{1,}").unwrap();
    let slug = re.replace_all(&slug, "-");

    slug.to_string()
}

// bson::DateTime -> Y-M-D
pub async fn bdt_to_ymd(bdt: mongodb::bson::DateTime) -> String {
    bdt.to_chrono().format(super::constant::DTF_YMD).to_string()
}

// bson::DateTime -> Y-M-D
pub async fn bdt_to_ymd8(bdt: mongodb::bson::DateTime) -> String {
    bdt.to_chrono()
        .with_timezone(&super::constant::TZ_FO_8.unwrap())
        .format(super::constant::DTF_YMD)
        .to_string()
}

// bson::DateTime -> Y-M-D H:M:S.Z
pub async fn bdt_to_ymdhmsz(bdt: mongodb::bson::DateTime) -> String {
    bdt.to_chrono().format(super::constant::DTF_YMDHMSZ).to_string()
}

// bson::DateTime -> Y-M-D H:M:S.Z
pub async fn bdt_to_ymdhmsz_8(bdt: mongodb::bson::DateTime) -> String {
    bdt.to_chrono()
        .with_timezone(&super::constant::TZ_FO_8.unwrap())
        .format(super::constant::DTF_YMDHMSZ)
        .to_string()
}
