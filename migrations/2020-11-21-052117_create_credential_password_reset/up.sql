CREATE TABLE credential_password_reset(
  credential_id INTEGER NOT NULL,
  secret TEXT NOT NULL,
  expired_at TIMESTAMP NOT NULL,
  CONSTRAINT credential_password_reset_pkey PRIMARY KEY (credential_id),
  CONSTRAINT credential_password_reset_secret_unique UNIQUE (secret),
  CONSTRAINT credential_password_reset_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES credential(id) ON DELETE CASCADE
);
