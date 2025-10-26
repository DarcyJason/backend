# Rust backend (Axum)

## Content

[Introduction](#introduction)

[Preparation](#preparation)

[API Book](/docs/api_book.md)

## Introduction

This is a Rust backend (Axum) with SurrealDB and Redis. It is a fully functional Auth system. Feel free to use it as a starting point for your own projects.

## Preparations

### 1. Install SurrealDB

```sh
docker run --name surrealdb \
  -p 10086:8000 \
  -v ~/surrealdb:/data \
  -d surrealdb/surrealdb:latest \
  start \
  --user root \
  --pass root \
  rocksdb:/data/mydatabase.db
```
You can download the [Surrealist](https://surrealdb.com/surrealist) to your local machine if you want to use SurrealDB in GUI.

### 2. Install SurrealDB Migrations

```sh
cargo install surrealdb-migrations
```

### 3. Run migrations

```sh
surrealdb-migrations apply
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

Configure the .env file.

### 6. Run the backend

```sh
cargo run
```
