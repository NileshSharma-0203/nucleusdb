# NucleusDB

NucleusDB is an educational relational database engine built in Rust to explore database internals including query parsing, execution, storage engines, transactions, WAL, buffer management, indexing, and MVCC.

This project was built from scratch to understand how real databases such as PostgreSQL, SQLite, and MySQL work internally.

---

# Features

## SQL Frontend
- SQL lexer/tokenizer
- SQL parser
- Abstract Syntax Tree (AST)
- Interactive REPL

## Query Execution
- CREATE TABLE
- INSERT
- SELECT
- WHERE filtering

## Query Planning
- Sequential scan planning
- Index scan planning
- EXPLAIN support
- Basic cost-based optimization

## Storage Engine
- Fixed-size pages (4096-byte pages)
- Disk manager
- Heap-file storage
- Binary row serialization
- Record IDs (RID)

## Buffer Management
- Buffer pool caching
- LRU page replacement
- Dirty-page flushing

## Indexing
- B+ tree infrastructure
- Indexed lookup
- Range scans

## Transactions
- BEGIN / COMMIT / ROLLBACK
- Transaction buffering
- Write-Ahead Logging (WAL)

## Recovery
- WAL recovery scanning
- Recovery analysis for committed transactions

## Concurrency Control
- Shared locks
- Exclusive locks
- Lock manager
- Deadlock detection

## MVCC
- Multi-Version Concurrency Control basics
- Snapshot visibility
- Versioned row reads

---

# Architecture

```text
SQL Query
   ↓
Lexer
   ↓
Parser
   ↓
AST
   ↓
Planner / Optimizer
   ↓
Executor
   ↓
Transaction Manager
   ↓
Buffer Pool
   ↓
Heap File / Pages
   ↓
Disk Manager
   ↓
Database File
```

---

# Tech Stack

- Rust
- Cargo
- Standard Library Collections & File APIs

---

# Project Goals

The goal of NucleusDB is to understand:
- database internals
- query execution
- storage engines
- transaction systems
- memory management
- concurrency control
- systems programming in Rust

This project is educational and intentionally focuses on learning database architecture rather than production deployment.

---

# Example Queries

```sql
CREATE TABLE users (id INT, name TEXT);

INSERT INTO users VALUES (1, 'Nilesh');

SELECT * FROM users;

BEGIN;
INSERT INTO users VALUES (2, 'Alex');
COMMIT;

EXPLAIN SELECT * FROM users WHERE id = 1;
```

---

# Running The Project

```bash
cargo run
```

---

# Future Improvements

- Disk-backed B+ trees
- SQL joins
- Query execution operators
- Better optimizer statistics
- TCP server support
- Parallel query execution
- Improved recovery mechanisms

---

# Learning References

This project was inspired by concepts from:
- PostgreSQL
- SQLite
- CMU Database Systems
- Database Internals by Alex Petrov

---

# Author

Nilesh Sharma
