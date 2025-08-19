-- Active: 1754301379973@@127.0.0.1@6500@e_commerce
-- Add up migration script here


CREATE TYPE product_status AS ENUM ('Inactive','Active');



CREATE TABLE
    IF NOT EXISTS products (
        id SERIAL UNIQUE,
        name VARCHAR(64) NOT NULL,
        description VARCHAR(128),
        status product_status NOT NULL DEFAULT 'Inactive',
        price NUMERIC(12,2) NOT NULL,

        stocks INT NOT NULL,
        created_by UUID NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now(),
        edited_by UUID,
        edited_at TIMESTAMP,

        PRIMARY KEY(id, name),
        FOREIGN KEY(created_by) REFERENCES users(id),
        FOREIGN KEY(edited_by) REFERENCES users(id)
    );

CREATE TABLE
    IF NOT EXISTS bundles (
        id SERIAL UNIQUE NOT NULL ,
        name VARCHAR(128) NOT NULL,
        price NUMERIC(12,2) NOT NULL,
        status product_status NOT NULL DEFAULT 'Inactive',
        created_by UUID NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now(),
        edited_by UUID,
        edited_at TIMESTAMP,

        PRIMARY KEY (id, name),
        FOREIGN KEY(created_by) REFERENCES users(id),
        FOREIGN KEY(edited_by) REFERENCES users(id)
    );


CREATE TABLE
    IF NOT EXISTS bundle_items (
        product_id INT NOT NULL,
        bundle_id INT NOT NULL,
        quantity INT NOT NULL,

        PRIMARY KEY (product_id, bundle_id),
        Foreign Key (product_id) REFERENCES products(id),
        Foreign Key (bundle_id) REFERENCES bundles(id)
    );



