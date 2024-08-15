-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name CHARACTER VARYING(150) NOT NULL,
    email CHARACTER VARYING(150) UNIQUE NOT NULL,
    password CHARACTER VARYING(150) NOT NULL,
    salary INTEGER NOT NULL,
    strikes INTEGER DEFAULT 0 NOT NULL,
    loan_limit INTEGER DEFAULT 15000 NOT NULL,
    goodwill INTEGER DEFAULT 0 NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
)