# Rust backend (Axum)

## Preparations

### 1. Install SurrealDB

```sh
docker run -d --name surrealdb -p 10086:8000 -v ~/surrealdb:/data surrealdb/surrealdb:latest start --user root --pass root rocksdb:/data/mydatabase.db
```
You can download the [Surrealist](https://surrealdb.com/surrealist) to your local machine for controlling SurrealDB in GUI.

### 2. Install SurrealDB Migrations

```sh
cargo install surrealdb-migrations
```

### 3. Run migrations

```sh
surrealdb-migrations run
```

### 4. Install Redis

```sh
docker run --name redis \
  -p 6379:6379 \
  -v ~/redis:/data \
  -d redis:latest
```

### 5. Configure .env

```sh
cp .env.example .env
```

Configure the RESEND_API_KEY in the .env file.

### 6. Run the backend

```sh
cargo run
```
