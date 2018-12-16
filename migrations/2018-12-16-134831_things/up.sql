-- Your SQL goes here
create table things
(
id          serial                   not null constraint things_pkey primary key,
name        varchar                  not null,
description text                     not null,
active      boolean default false    not null,
sometext    text    default ''::text not null,
item        integer default 0        not null
);
