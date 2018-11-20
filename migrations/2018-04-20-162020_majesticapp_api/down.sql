-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS categories_of_trash;
DROP TABLE IF EXISTS signin_logs;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS packages_of_supplier;
DROP TABLE IF EXISTS suppliers;
DROP TABLE IF EXISTS supplier_reviews;
DROP TABLE IF EXISTS partners;
DROP TABLE IF EXISTS partner_reviews;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS reports_to_block;

DROP TYPE IF EXISTS status_transaction;
DROP TYPE IF EXISTS user_role;

DROP TABLE IF EXISTS provinces;
DROP TABLE IF EXISTS districts;
DROP TABLE IF EXISTS sub_districts;
DROP TABLE IF EXISTS villages;