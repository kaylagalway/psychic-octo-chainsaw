CREATE TABLE sessions (
    token VARCHAR PRIMARY KEY,
    exp_date INTEGER NOT NULL,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id)
)