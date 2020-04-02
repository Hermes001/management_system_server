table! {
    assortment (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    commodity (id) {
        id -> Text,
        assortment_id -> Text,
        model -> Nullable<Text>,
        description -> Nullable<Text>,
        price_retrieve -> Int4,
        price_sale -> Nullable<Int4>,
        user_id -> Text,
        memory -> Nullable<Int2>,
        harddisk -> Nullable<Int2>,
        harddisk_type_id -> Nullable<Text>,
        gpu -> Nullable<Text>,
        cpu -> Nullable<Text>,
        price_retrieve_external -> Nullable<Int4>,
        price_sale_external -> Nullable<Int4>,
        photo_url -> Nullable<Text>,
        quantity -> Int2,
    }
}

table! {
    harddisk_type (id) {
        id -> Text,
        name -> Nullable<Text>,
    }
}

table! {
    user (id) {
        id -> Text,
        user_name -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(assortment, commodity, harddisk_type, user,);
