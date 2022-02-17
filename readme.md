## Actix Rust Boilerplate

### Development
```zsh
cargo watch -x run
```

### diesel setup
```zsh
# in root directory
diesel setup
# write what you want table specification sql in migrations/up.sql
# write table drop sql in migrations/down.sql
# define schema file in diesel.toml
[print_schema]
file = "src/database/schema.rs"
# run up.sql
diesel migration run
# run down.sql and then up.sql
diesel migration redo
```