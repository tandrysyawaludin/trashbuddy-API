-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS categories_of_trash;
DROP TABLE IF EXISTS signin_log;
DROP TYPE IF EXISTS user_role;
DROP TABLE IF EXISTS packages_of_supplier;
DROP TABLE IF EXISTS suppliers;
DROP TABLE IF EXISTS supplier_review;
DROP TABLE IF EXISTS partners;
DROP TABLE IF EXISTS partner_review;
DROP TABLE IF EXISTS transactions;
DROP TYPE IF EXISTS status_transaction;
DROP TABLE IF EXISTS report_to_block;