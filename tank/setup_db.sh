#!/bin/bash

# Start pg docker container
cd pg-docker/
docker-compose down
docker-compose up -d
cd ..

# Wait for container to start
sleep 1

# Create the database
sqlx migrate run --source migrations --database-url postgres://toodaloo:password@localhost:5432/development_db

# Generate the query files
cargo sqlx prepare --database-url postgres://toodaloo:password@localhost:5432/development_db --workspace
