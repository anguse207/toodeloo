-- Add migration script here

-- Create the Users table
CREATE TYPE oauth_provider AS ENUM ('discord', 'google', 'github', 'apple');
CREATE TABLE Users (
    id UUID PRIMARY KEY,
    oauth_id TEXT NOT NULL,
    oauth_provider oauth_provider NOT NULL,
    nickname TEXT NOT NULL UNIQUE,
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
