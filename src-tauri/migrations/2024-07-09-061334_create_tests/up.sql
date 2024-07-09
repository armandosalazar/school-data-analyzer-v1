-- Your SQL goes here
create table tests (
  id integer not null primary key autoincrement,
  name text not null,
  created_at timestamp not null default current_timestamp
);