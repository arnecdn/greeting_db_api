-- Add migration script here
CREATE TABLE IF NOT EXISTS greeting
(
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    external_reference TEXT  NOT NULL,
    message_id UUID UNIQUE  NOT NULL,
    created    TIMESTAMP    DEFAULT CURRENT_TIMESTAMP NOT NULL,
    message    JSONB       NOT NULL
);