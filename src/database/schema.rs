table! {
    categories_of_trash (name) {
        name -> Varchar,
        description -> Text,
    }
}

table! {
    packages_of_supplier (id) {
        id -> Int4,
        weight -> Int4,
        shipping_fee -> Int4,
        price -> Int4,
        category_of_trash_id -> Int4,
        supplier_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    partners (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Varchar,
        area -> Varchar,
        category_of_trash_id -> Nullable<Array<Text>>,
        machine_code -> Nullable<Varchar>,
        is_live -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    reports_to_block (id) {
        id -> Int4,
        target_user -> Int4,
        comment -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    signin_logs (id) {
        id -> Int4,
        user_id -> Int4,
        user_group -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    suppliers (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Varchar,
        area -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        supplier_id -> Int4,
        partner_id -> Int4,
        id_package_of_supplier -> Int4,
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    supplier_reviews (id) {
        id -> Int4,
        score -> Int4,
        comment -> Varchar,
        transactions_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    partner_reviews (id) {
        id -> Int4,
        score -> Int4,
        comment -> Varchar,
        transactions_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(districts -> provinces(province_id));
table! {
    provinces (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(sub_districts -> districts(district_id));
table! {
    districts (id) {
        id -> Varchar,
        name -> Varchar,
        province_id -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(provinces, districts, sub_districts);
table! {
    sub_districts (id) {
        id -> Varchar,
        name -> Varchar,
        district_id -> Varchar,
    }
}

// allow_tables_to_appear_in_same_query!(
//     categories_of_trash,
//     district,
//     packages_of_supplier,
//     partner_review,
//     partners,
//     province,
//     report_to_block,
//     signin_log,
//     sub_district,
//     supplier_review,
//     suppliers,
//     transactions,
// );
