-- Add migration script here
CREATE TABLE accounts (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name            TEXT        NOT NULL,
    currency        CHAR(3)     NOT NULL DEFAULT 'GBP',
    allow_negative  BOOLEAN     NOT NULL DEFAULT false,  -- only systems accounts may go below zero
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE transactions (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    kind         TEXT        NOT NULL CHECK (kind IN ('transfer', 'reversal')),
    reverses_id  BIGINT      UNIQUE REFERENCES transactions(id),  -- a transaction can be reversed once
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE entries (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    transaction_id  BIGINT      NOT NULL REFERENCES transactions(id),
    account_id      BIGINT      NOT NULL REFERENCES accounts(id),
    amount          BIGINT      NOT NULL CHECK (amount <> 0),       -- pence
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX entries_account_id_id ON entries (account_id, id) INCLUDE (amount);
CREATE INDEX entries_transaction_id ON entries (transaction_id);