CREATE TABLE public.boards (
    id bigint NOT NULL,
    name text NOT NULL,
    description text NOT NULL
);
CREATE TABLE public.posts (
    id bigint NOT NULL,
    body text NOT NULL,
    parent bigint,
    board_id bigint NOT NULL
);
