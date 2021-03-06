CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   profile_picture VARCHAR NOT NULL,
   username VARCHAR NOT NULL  UNIQUE,
   email VARCHAR(254) NOT NULL UNIQUE,
   phone VARCHAR NOT NULL UNIQUE,
   password VARCHAR NOT NULL,
   role VARCHAR DEFAULT 'Player' NOT NULL,
   is_active BOOLEAN NOT NULL DEFAULT TRUE,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)