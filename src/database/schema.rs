// table! {
//     categories_of_trash (id) {
//         id -> Int4,
//         name -> Bpchar,
//     }
// }

// table! {
//     packages_of_supplier (id) {
//         id -> Int4,
//         weight -> Int4,
//         shipping_fee -> Int4,
//         price -> Int4,
//         category_of_trash_id -> Int4,
//         supplier_id -> Int4,
//         created_at -> Nullable<Timestamp>,
//     }
// }

// table! {
//     partners (id) {
//         id -> Int4,
//         name -> Bpchar,
//         password -> Bpchar,
//         phone_number -> Bpchar,
//         area -> Bpchar,
//         email -> Bpchar,
//         created_at -> Nullable<Timestamp>,
//     }
// }

// table! {
//     report_to_block (id) {
//         id -> Int4,
//         target_user -> Int4,
//         comment -> Bpchar,
//     }
// }

table! {
    use diesel::sql_types::*;
    use super::User_role;
    signin_log (id) {
        id -> Int4,
        user_id -> Int4,
        user_group -> User_role,
        created_at -> Nullable<Timestamp>,
        optional_data -> Nullable<Json>,
    }
}

table! {
    suppliers (id) {
        id -> Int4,
        name -> Bpchar,
        email -> Bpchar,
        password -> Bpchar,
        phone_number -> Bpchar,
        created_at -> Nullable<Timestamp>,        
    }
}

// table! {
//     use diesel::sql_types::*;
//     use super::Status_transaction;
//     transactions (id) {
//         id -> Int4,
//         supplier_id -> Int4,
//         partner_id -> Int4,
//         packages_of_supplier_id -> Int4,
//         status -> Status_transaction,
//         created_at -> Nullable<Timestamp>,
//         supplier_review -> Nullable<Json>,
//         partner_review -> Nullable<Json>,
//     }
// }

// allow_tables_to_appear_in_same_query!(
//     categories_of_trash,
//     packages_of_supplier,
//     partners,
//     report_to_block,
//     signin_log,
//     suppliers,
//     transactions,
// );

use signin_module::signin_log::{User_role};
// use super::transaction::{Status_transaction};
