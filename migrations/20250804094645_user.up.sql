-- Active: 1754301379973@@127.0.0.1@6500@e_commerce
-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role as ENUM ('Regular', 'Worker', 'Admin');


CREATE TABLE
    IF NOT EXISTS users (
        id UUID UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
        email VARCHAR(64) UNIQUE NOT NULL,
        password VARCHAR(128) NOT NULL,
        username VARCHAR(32) UNIQUE NOT NULL,
        
        first_name VARCHAR(64) NOT NULL,
        last_name VARCHAR(64) NOT NULL,

        location VARCHAR(128) NOT NULL,
        phone_no VARCHAR(20) NOT NULL,

        role user_role NOT NULL DEFAULT 'Regular'
    );

