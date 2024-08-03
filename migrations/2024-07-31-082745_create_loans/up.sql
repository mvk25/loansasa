-- Your SQL goes here
CREATE TYPE loan_type AS ENUM ('personal', 'auto', 'student', 'mortgage', 'payday', 'msme');

CREATE TABLE loans (
    id SERIAL PRIMARY KEY,
    loan loan_type NOT NULL,
    amount INTEGER NOT NULL,
    upper_limit INTEGER default 15000 NOT NULL,
    status BOOLEAN DEFAULT false NOT NULL,
    deadline TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    users_id INTEGER NOT NULL REFERENCEs users (id),
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
