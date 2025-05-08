-- Add migration script here

-- Create the Users table
CREATE TABLE Users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    pass TEXT NOT NULL,
    deleted_time BIGINT NOT NULL DEFAULT 0
);

-- Create the Tokens table
CREATE TABLE Tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    expiry BIGINT NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create the Lists table
CREATE TABLE Lists (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    deleted_time BIGINT NOT NULL DEFAULT 0
);

-- Create the Tasks table
CREATE TABLE Tasks (
    id UUID PRIMARY KEY,
    list_id UUID NOT NULL REFERENCES Lists(id) ON DELETE CASCADE,
    origin_time BIGINT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    snoozed_until BIGINT NOT NULL DEFAULT 0,
    deleted_time BIGINT NOT NULL DEFAULT 0
);
