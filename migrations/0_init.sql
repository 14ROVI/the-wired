CREATE TABLE public.posts
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    author bigint NOT NULL,
    content text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.posts
    OWNER to the_wired;