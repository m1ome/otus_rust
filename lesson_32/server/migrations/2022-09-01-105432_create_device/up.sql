-- Your SQL goes here
CREATE TABLE device (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    state JSONB NOT NULL,
    room_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (room_id) REFERENCES room(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX ON device (room_id, name);