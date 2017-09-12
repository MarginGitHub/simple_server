// table! {
//     user (id) {
//         id -> Nullable<Integer>,
//         uid -> Integer,
//         mobile -> Text,
//         password -> Text,
//         nick_name -> Nullable<Text>,
//         avatar_url -> Nullable<Text>,
//         last_login_time -> Nullable<Integer>,
//     }
// }
infer_schema!("env:DATABASE_URL");