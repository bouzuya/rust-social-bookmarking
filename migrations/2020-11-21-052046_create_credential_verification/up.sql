CREATE TABLE credential_verification(
  credential_id INTEGER NOT NULL,
  secret TEXT NOT NULL,
  expired_at TIMESTAMP NOT NULL,
  CONSTRAINT credential_verification_pkey PRIMARY KEY (credential_id),
  CONSTRAINT credential_verification_secret_unique UNIQUE (secret),
  CONSTRAINT credential_verification_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES credential(id)
);
