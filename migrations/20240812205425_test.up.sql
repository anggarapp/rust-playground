-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS test (
        "id" SERIAL PRIMARY KEY NOT NULL,
        "place" varchar NOT NULL,
        "created_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW (),
            "updated_at" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW ()
    );