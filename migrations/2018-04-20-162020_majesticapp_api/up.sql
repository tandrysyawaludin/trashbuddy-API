-- Your SQL goes here
CREATE TABLE categories_of_trash (
    id SERIAL PRIMARY KEY,
    name CHARACTER(20) NOT NULL
);

CREATE TYPE user_role AS ENUM ('supplier', 'partner');
CREATE TABLE signin_log (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    user_group user_role NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    email CHARACTER(50) NOT NULL,
    password CHARACTER(20) NOT NULL,
    phone_number CHARACTER(20) NOT NULL,
    area CHARACTER(100) NOT NULL,    
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE packages_of_supplier (
    id SERIAL PRIMARY KEY,
    weight integer NOT NULL,
    shipping_fee integer NOT NULL,
    price integer NOT NULL,
    category_of_trash_id integer NOT NULL,
    supplier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    email CHARACTER(50) NOT NULL,
    password CHARACTER(20) NOT NULL,
    phone_number CHARACTER(20) NOT NULL,
    area CHARACTER(20) NOT NULL,
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE supplier_review (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment CHARACTER(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
);

CREATE TABLE partner_review (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment CHARACTER(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
);

CREATE TYPE status_transaction AS ENUM ('pending', 'process', 'success');
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    supplier_id integer NOT NULL,
    partner_id integer NOT NULL,
    packages_of_supplier_id integer NOT NULL,
    status status_transaction NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE report_to_block (
    id SERIAL PRIMARY KEY,
    target_user integer NOT NULL,
    comment CHARACTER(100) NOT NULL
);
