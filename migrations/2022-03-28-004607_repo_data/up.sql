-- Your SQL goes here
create table packages
(
    name             text   not null
        constraint packages_pk
            primary key,
    version          text   not null,
    epoch            int    not null,
    description      text   not null,
    groups           text[],
    url              text   not null,
    license          text[],
    depends          text[],
    optional_depends text[],
    make_depends     text[],
    provides         text[] not null,
    conflicts        text[],
    replaces         text[],
    maintainers      text[] not null,
    repo             text   not null
);

create unique index packages_name_uindex
    on packages (name);