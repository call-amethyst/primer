CREATE TABLE sessions (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  refreshed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '30 DAYS',
  access_expires_at TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '30 MINUTE',

  access_token_hash BYTEA NOT NULL UNIQUE,
  refresh_token_hash BYTEA NOT NULL UNIQUE
)
