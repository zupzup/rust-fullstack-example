CREATE TABLE IF NOT EXISTS todo
(
    id SERIAL PRIMARY KEY NOT NULL,
    name    VARCHAR(255),
    ordering INT DEFAULT 0 NOT NULL,
    checked boolean DEFAULT false
);

