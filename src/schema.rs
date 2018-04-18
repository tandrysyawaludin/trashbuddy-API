table! {
    category_of_trash (id) {
        id -> Int4,
        name -> Bpchar,
    }
}

table! {
    customers (id) {
        id -> Int4,
        name -> Bpchar,
        email -> Bpchar,
        password -> Bpchar,
        phone_number -> Int4,
    }
}

table! {
    packet_of_partner (id) {
        id -> Int4,
        category_of_trash -> Int4,
        min_weight -> Int4,
        price -> Int4,
    }
}

table! {
    partners (id) {
        id -> Int4,
        name -> Bpchar,
        phone_number -> Int4,
        area -> Bpchar,
        packet -> Int4,
        email -> Bpchar,
        password -> Bpchar,
    }
}

allow_tables_to_appear_in_same_query!(
    category_of_trash,
    customers,
    packet_of_partner,
    partners,
);
