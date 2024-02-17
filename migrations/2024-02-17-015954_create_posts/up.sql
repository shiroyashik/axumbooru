CREATE TABLE posts (
    id integer PRIMARY KEY,
    user_id integer REFERENCES users(id) ON DELETE SET NULL,
    creation_time timestamp without time zone NOT NULL,
    last_edit_time timestamp without time zone,
    safety character varying(32) NOT NULL,
    type character varying(32) NOT NULL,
    checksum character varying(64) NOT NULL,
    source character varying(2048),
    file_size bigint,
    image_width integer,
    image_height integer,
    "mime-type" character varying(32) NOT NULL,
    version integer NOT NULL,
    flags character varying(32),
    checksum_md5 character varying(32)
)