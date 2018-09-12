-- Your SQL goes here
CREATE TABLE categories_of_trash (
    name VARCHAR(50) PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TYPE user_role AS ENUM ('supplier', 'partner');
CREATE TABLE signin_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL,
    user_group user_role NOT NULL,
    is_valid BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    phone_number VARCHAR(20) UNIQUE NOT NULL,
    area VARCHAR(100) NOT NULL,    
    address TEXT NOT NULL,    
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    sender INTEGER NOT NULL,
    receiver INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    phone_number VARCHAR(20) UNIQUE NOT NULL,
    area VARCHAR(100) NOT NULL,
    address TEXT NOT NULL,
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    category_of_trash_id TEXT[],
    machine_code VARCHAR (10),
    is_live BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE supplier_reviews (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
);

CREATE TABLE partner_reviews (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
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

CREATE TYPE status_transaction AS ENUM ('pending', 'process', 'success');
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    supplier_id integer NOT NULL,
    partner_id integer NOT NULL,
    id_package_of_supplier integer NOT NULL,
    status status_transaction NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE reports_to_block (
    id SERIAL PRIMARY KEY,
    target_user integer NOT NULL,
    comment VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()    
);

-- CREATE TABLE provinces (
--     id VARCHAR(2) PRIMARY KEY,
--     name VARCHAR(50) NOT NULL
-- );

-- CREATE TABLE districts (
--     id VARCHAR(4) PRIMARY KEY,    
--     name VARCHAR(50) NOT NULL,
--     province_id VARCHAR(2) NOT NULL
-- );

-- CREATE TABLE sub_districts (
--     id VARCHAR(6) PRIMARY KEY,    
--     name VARCHAR(50) NOT NULL,
--     district_id VARCHAR(4) NOT NULL
-- );

-- CREATE TABLE villages (
--     id VARCHAR(10) PRIMARY KEY,
--     name VARCHAR(50) NOT NULL,
--     sub_district_id VARCHAR(6) NOT NULL
-- );
