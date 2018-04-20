-- Your SQL goes here
CREATE TABLE categories_of_trash (
    id SERIAL PRIMARY KEY,
    name CHARACTER(20) NOT NULL
);

CREATE TYPE user_role AS ENUM ('customer', 'partner');
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
    phone_number integer NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE packages_of_partner (
    id SERIAL PRIMARY KEY,
    category_of_trash_id integer NOT NULL,
    min_weight integer NOT NULL,
    price integer NOT NULL,
    partner_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    password CHARACTER(20) NOT NULL,
    phone_number integer NOT NULL,
    area CHARACTER(20) NOT NULL,
    email CHARACTER(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);