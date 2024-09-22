-- Your SQL goes here
CREATE TYPE loan_type AS ENUM ('personal', 'auto', 'student', 'mortgage', 'payday', 'msme');

CREATE TYPE status_type AS ENUM ('pending', 'active', 'paid', 'overdue');

CREATE TABLE loans (
    id SERIAL PRIMARY KEY,
    loan loan_type NOT NULL,
    amount INTEGER NOT NULL,
    upper_limit INTEGER default 15000 NOT NULL,
    status status_type DEFAULT 'pending' NOT NULL,
    loanterm INTEGER NOT NULL,
    deadline TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    users_id INTEGER NOT NULL REFERENCEs users (id),
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);
