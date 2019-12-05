table! {
    databases (id) {
        id -> Nullable<Integer>,
    }
}

table! {
    post (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        post_date -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    databases,
    post,
);
