CREATE TABLE users (
  id UUID PRIMARY KEY,
  name varchar(255) NOT NULL,
  riot_name varchar(255) NOT NULL,
  password_hash VARCHAR(255) NOT NULL
);
