// @generated automatically by Diesel CLI.

diesel::table! {
    sales (id) {
        id -> Nullable<Integer>,
        shop_id -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    shops (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sales,
    shops,
);
