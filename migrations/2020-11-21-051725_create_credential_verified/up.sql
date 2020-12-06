CREATE TABLE credential_verified(
  credential_id INTEGER NOT NULL,
  verified_at TIMESTAMP NOT NULL,
  CONSTRAINT credential_verified_pkey PRIMARY KEY (credential_id),
  CONSTRAINT credential_verified_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES credential(id) ON DELETE CASCADE
);
