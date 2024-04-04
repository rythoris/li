CREATE TABLE IF NOT EXISTS links (
    id          SERIAL       PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    url         TEXT         NOT NULL UNIQUE,
    tags        TEXT[]
);
