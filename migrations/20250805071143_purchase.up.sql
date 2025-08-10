-- Active: 1754301379973@@127.0.0.1@6500@e_commerce
-- Add up migration script here


CREATE TYPE item_type AS ENUM ('product', 'bundle');
CREATE TYPE purchase_status AS ENUM ('packing', 'arriving', 'recieved', 'cancelled');

CREATE TABLE 
    IF NOT EXISTS carts (
        user_id UUID NOT NULL,
        item_id INTEGER NOT NULL,
        item_type item_type NOT NULL,
        quantity SMALLINT NOT NULL,
        time_created TIMESTAMP NOT NULL,

        PRIMARY KEY(user_id, item_id, item_type),
        Foreign Key (user_id) REFERENCES users(id)
    );


CREATE TABLE 
    IF NOT EXISTS purchases (
        id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL,

        item_id INTEGER NOT NULL,
        item_type item_type NOT NULL,

        status purchase_status NOT NULL DEFAULT 'packing',
        time_purchase TIMESTAMP NOT NULL,

        PRIMARY KEY(id),
        Foreign Key (user_id) REFERENCES users(id)
    );

CREATE INDEX idx_purchase_id ON purchases(id);