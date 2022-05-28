# Key-Value Database

## Features

-> append-only structure

-> in-memory key storage for faster lookup

-> takes in any values for key:value

-> Basic operations: GET, UPDATE, DELETE, INSERT

## RUN

```
Usage:
    kv_mem FILE get KEY
    kv_mem FILE delete KEY
    kv_mem FILE insert KEY VALUE
    kv_mem FILE update KEY VALUE
```

```
cargo run -- <dbfile> <args>
```