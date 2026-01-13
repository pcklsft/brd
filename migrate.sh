# !/bin/sh
# I just do this for ergonomics so that i have a file i can look at in my editor to see the database schema
DATABASE_URL=$(grep DATABASE_URL .env | cut -d "=" -f2)
DATABASE_URL="${DATABASE_URL%\"}"
DATABASE_URL="${DATABASE_URL#\"}"

sqlx migrate run

pg_dump -sOxc --exclude-table="_sqlx_migrations" $DATABASE_URL | awk 'RS="";/CREATE TABLE[^;]*;/' | cat >  queries/schema.sql
# pg_dump -s $DATABASE_URL | cat
