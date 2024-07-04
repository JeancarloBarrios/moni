CREATE TABLE prompt (
    id uuid primary key default gen_random_uuid(),
    prompt TEXT NOT NULL,
    code_name VARCHAR(100) NOT NULL UNIQUE
);
