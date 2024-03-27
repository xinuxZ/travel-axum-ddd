-- add the article tables
create table if not exists articles
(
    id          bigint generated by default as identity,
    title       varchar     not null default '',
    body        varchar     not null default '',
    description varchar     not null default '',
    slug        varchar     not null default '',
    user_id     bigint      not null references users (id) on delete cascade,
    created_at  timestamptz not null default current_timestamp,
    updated_at  timestamptz not null default current_timestamp
);

alter table articles
    add constraint articles_id_pk primary key (id);

create index if not exists articles_slug_idx on articles (slug);

-- add a table for referencing tags, indexed on the tag name
create table if not exists tags
(
    id         bigint generated by default as identity,
    tag        varchar     not null default '',
    created_at timestamptz not null default current_timestamp
);

alter table tags
    add constraint tags_id_pk primary key (id);

create index if not exists tags_tag_idx on tags (tag);

-- add the related entities
create table if not exists article_tags
(
    id         bigint generated by default as identity,
    tag_id     bigint      not null references tags (id) on delete cascade,
    article_id bigint      not null references articles (id) on delete cascade,
    created_at timestamptz not null default current_timestamp
);

alter table article_tags
    add constraint article_tags_id_pk primary key (id);