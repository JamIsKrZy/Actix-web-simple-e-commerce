-- Add down migration script here

DROP TABLE IF EXISTS purchases;
DROP TABLE IF EXISTS carts;
DROP TYPE IF EXISTS item_type;
DROP TYPE IF EXISTS purchase_status;
