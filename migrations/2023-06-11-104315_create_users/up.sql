-- Your SQL goes here
CREATE TABLE users (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR(24) NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  password VARCHAR(24) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)