CREATE TABLE users (
    id integer PRIMARY KEY,
    username character varying(50) UNIQUE,
    password character varying(128) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    email character varying(64),
    access_level character varying(32) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone,
    avatar_style character varying(32) NOT NULL
)