table! {
    categories_of_trash (id) {
        id -> Int4,
        name -> Bpchar,
    }
}

table! {
    packages_of_partner (id) {
        id -> Int4,
        category_of_trash_id -> Int4,
        min_weight -> Int4,
        price -> Int4,
        partner_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    partners (id) {
        id -> Int4,
        name -> Bpchar,
        password -> Bpchar,
        phone_number -> Int4,
        area -> Bpchar,
        email -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::types::String;
    use super::User_role;
    signin_log (id) {
        id -> Int4,
        user_id -> Int4,
        user_group -> User_role,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    suppliers (id) {
        id -> Int4,
        name -> Bpchar,
        email -> Bpchar,
        password -> Bpchar,
        phone_number -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories_of_trash,
    packages_of_partner,
    partners,
    signin_log,
    suppliers,
);
