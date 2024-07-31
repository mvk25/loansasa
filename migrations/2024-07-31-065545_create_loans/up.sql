-- Your SQL goes here
CREATE TYPE loan_type AS ENUM ('personal', 'auto', 'student', 'mortgage', 'payday', 'msme');

CREATE TABLE loans (
    id SERIAL PRIMARY KEY,
    loan loan_type,
    upper_limit INTEGER NOT NULL,
    deadline TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    user_id INTEGER REFERENCEs users (id),
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
