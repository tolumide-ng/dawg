-- name should be derived from the env
SELECT 'CREATE DATABASE dawgie'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'dawgie')\gexec