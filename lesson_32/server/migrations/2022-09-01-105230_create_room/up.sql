-- Your SQL goes here
CREATE TABLE room (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    house_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (house_id) REFERENCES house(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX ON room (house_id, name);