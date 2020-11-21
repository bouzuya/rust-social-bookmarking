CREATE TABLE "user"(
  id INTEGER NOT NULL,
  key TEXT NOT NULL,
  CONSTRAINT user_pkey PRIMARY KEY (id),
  CONSTRAINT user_key_unique UNIQUE (key)
);
