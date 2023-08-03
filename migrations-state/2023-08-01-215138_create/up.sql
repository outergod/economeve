-- Your SQL goes here
CREATE TABLE characters (
       id INTEGER PRIMARY KEY NOT NULL,
       name TEXT UNIQUE NOT NULL
);

CREATE TABLE tokens (
       character_id INTEGER PRIMARY KEY NOT NULL,
       access_token TEXT NOT NULL,
       refresh_token TEXT NOT NULL,
       expiry DATETIME NOT NULL,
       FOREIGN KEY (character_id) REFERENCES characters (id)
);
