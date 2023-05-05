let img_name = params[0];

let img_ext = img_name.sub_string(-4);
img_name.pop(4);

switch params.len {
    2 => {
        let thumbnail = params[1];
        switch img_name {
            "rusthub" => "../" + img_name + thumbnail + img_ext,
            _ => img_name + thumbnail + img_ext
        }
    },
    1 => {
        switch img_name {
            "rusthub" => "../" + img_name + img_ext,
            _ => img_name + img_ext
        }
    },
    _ => "参数错误！",
}
