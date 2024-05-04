# Running

Start a postgres docker container

```bash
$ docker run --name -p5432:5432 some-postgres -e POSTGRES_DB=movies -e POSTGRES_PASSWORD=postgres -d postgres
```

[Install Rust](https://www.rust-lang.org/tools/install)

```bash
user@populate_db$ cargo install sqlx-cli
user@populate_db$ sqlx migrate run
user@populate_db$ cargo run
```

