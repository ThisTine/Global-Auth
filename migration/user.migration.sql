CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
SELECT uuid_generate_v1();
create table "user"(
    id uuid NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    username text not null unique,
    email text  not null unique ,
    password text not null
)