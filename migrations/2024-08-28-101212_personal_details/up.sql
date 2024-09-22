-- Your SQL goes here
CREATE TYPE employment_type AS ENUM ('full-time', 'part-time', 'self-employed');

CREATE TABLE personal_details (
    id SERIAL PRIMARY KEY,
    users_id INTEGER NOT NULL REFERENCES users(id),
    phone_number INTEGER NOT NULL,
    date_of_birth TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    city CHARACTER VARYING(150) NOT NULL,
    employment employment_type NOT NULL,
    office_number INTEGER NOT NULL,
    monthly_salary INTEGER NOT NULL,
    company_name CHARACTER VARYING(150) NOT NULL,
    office_email_ID INTEGER NOT NULL
);