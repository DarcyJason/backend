# Rust 后端（Axum）

[简体中文](./README_zh_CN.md) | [English](./README.md)

## 目录

[介绍](#介绍)

[准备工作](#准备工作)

[API 文档](./docs/zh-CN/api_book.md)

## 介绍

这是一个基于 Rust (Axum) 的后端，使用了 SurrealDB 和 Redis。它是一个功能齐全的认证系统。您可以随意将其用作您自己项目的起点。

## 准备工作

### 1. 安装 SurrealDB

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
如果您想在 GUI 中使用 SurrealDB，可以下载 [Surrealist](https://surrealdb.com/surrealist) 到您的本地计算机。

### 2. 安装 SurrealDB Migrations

```sh
cargo install surrealdb-migrations
```

### 3. 运行迁移

```sh
surrealdb-migrations apply
```

### 4. 安装 Redis

```sh
docker run --name redis \
  -p 6379:6379 \
  -v ~/redis:/data \
  -d redis:latest
```

### 5. 配置 .env

```sh
cp .env.example .env
```

配置 .env 文件。

### 6. 运行后端

```sh
cargo run
```