#!/bin/bash

CONTAINER_NAME="football_manager_db"
DB_USER="user"
DB_NAME="football_manager"
DB_PORT="5433"
SQL_FILE="./database/schema.sql"

if [ ! -f "$SQL_FILE" ]; then
    echo "SQL file $SQL_FILE not found!"
    exit 1
fi

if ! docker ps | grep -q "$CONTAINER_NAME"; then
    echo "Container $CONTAINER_NAME is not running!"
    exit 1
fi

echo "Dropping and reinitializing tables in $DB_NAME on port $DB_PORT..."
docker exec -i $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME -p 5432 < $SQL_FILE

if [ $? -eq 0 ]; then
    echo "Tables reinitialized successfully."
else
    echo "Failed to reinitialize tables."
    exit 1
fi

