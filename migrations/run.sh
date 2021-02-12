#!/usr/bin/env bash
export DATABASE_URL=postgres://chatty:chatty@database/chatty
sqlx migrate run
exec "$@"
