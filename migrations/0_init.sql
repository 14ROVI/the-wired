CREATE TABLE users
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    username character varying(15) NOT NULL,
    display_name character varying(15) NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    avatar_ref text,
    CONSTRAINT users_pkey PRIMARY KEY (display_name),
    CONSTRAINT users_id_key UNIQUE (id),
    CONSTRAINT users_username_key UNIQUE (username),
    CONSTRAINT users_username_check CHECK (username::text ~ '^[a-z0-9_]+$'::text)
)

ALTER TABLE IF EXISTS users
    OWNER to the_wired;


CREATE TABLE posts
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    author bigint NOT NULL,
    content text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,,
    CONSTRAINT posts_pkey PRIMARY KEY (id),
    CONSTRAINT posts_author_fkey FOREIGN KEY (author)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

ALTER TABLE IF EXISTS posts
    OWNER to the_wired;
