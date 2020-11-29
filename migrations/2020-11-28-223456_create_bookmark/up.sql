CREATE TABLE bookmark(
  id INTEGER NOT NULL,
  key TEXT NOT NULL,
  user_id INTEGER NOT NULL,
  url TEXT NOT NULL,
  comment TEXT NOT NULL,
  title TEXT NOT NULL,
  CONSTRAINT bookmark_pkey PRIMARY KEY (id),
  CONSTRAINT bookmark_key_unique UNIQUE (key),
  CONSTRAINT bookmark_use_id_url_unique UNIQUE (user_id, url),
  CONSTRAINT bookmark_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id)
);
