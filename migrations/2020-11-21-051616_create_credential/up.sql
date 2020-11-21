CREATE TABLE credential(
  id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  mail_address TEXT NOT NULL,
  password TEXT NOT NULL,
  status INTEGER NOT NULL,
  CONSTRAINT credential_pkey PRIMARY KEY (id),
  CONSTRAINT credential_mail_address_unique UNIQUE (mail_address),
  CONSTRAINT credential_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id)
);
