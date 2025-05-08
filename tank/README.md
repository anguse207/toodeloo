# Useful commands for SQLX

### Dependencies

`$ cargo install sqlx-cli`

Define environment variable `DATABASE_URL`. Full static path seems to work in this case...

Example: `DATABASE_URL=sqlite:///home/gus/repos/clarifi/db/db.sqlite`

### Create a migration file

# `$ sqlx migrate add <migration_name>`

### Remove existing db

```bash
rm -f <static_db_path>
touch <static_db_path>
```

### Run the migration and create query data

```bash
# Create the database
sqlx migrate run --source migrations --database-url postgres://toodaloo:password@localhost:5432/development_db

# Generate the query data
cargo sqlx prepare --database-url postgres://toodaloo:password@localhost:5432/development_db --workspace
```