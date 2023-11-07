set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE poli;
    CREATE USER poli WITH ENCRYPTED PASSWORD 'poli453';
    GRANT ALL PRIVILEGES ON DATABASE new_user_name TO database_nam;
EOSQL