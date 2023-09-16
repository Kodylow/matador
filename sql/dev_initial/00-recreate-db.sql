-- DEV ONLY -- Brute Force DROP DB (for local dev and unit testing only)
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE usename = 'app_user'
    OR datname = 'app_db';
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;
-- DEV ONLY -- Dev only password for local dev and unit testing only
CREATE USER app_user WITH PASSWORD 'app_pwd';
CREATE DATABASE app_db WITH OWNER app_user ENCODING = 'UTF-8';