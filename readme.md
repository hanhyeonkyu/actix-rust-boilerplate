# Backend with Actix Web Boilerplate 
##### Created: February 16, 2022 1:02 PM
##### who: Alex
---
### Development
  ```zsh
  # for develop with watch
  cargo watch -x run
  # just run
  cargo run
  ```

### Pre-Requirements
- [x] Rust setup
    ```zsh
    # install
    brew install rustup
    rustup-init
    # verify
    rustc --version
    ```

- [x] [Actix](https://actix.rs/)
    
    ```zsh
    cargo new actix-web
    cd actix-web
    # In Cargo.toml file
    [dependencise]
    # if you type it, automatically check library and can use it.
    actix-web = "3"
    ```
- [x] diesel setup(Now no need)
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



## Purpose

Based on C++, C#, and C-like languages ​​with good performance, new languages ​​such as Go and Rust are growing to replace web frameworks.

[Both Go and Rust, which are new languages, have good performance, but among them, Rust is evaluated as overwhelmingly good.](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/rust-go.html)

So, while creating a web framework using Rust, I will try to write boilerplate so that it can be used for future development.

## Process

1. FileSystem
    1. The file structure of rust is called through `mod`. To load the contents of the `parent directory` from the .rs file of the `child directory`, you must define the directory structure through mod at the top to use it.(`main.rs`)
    2. `src/main.rs` - Registers server settings (port, cors,...) as the start of actix_web.
    3. `src/database.rs` - `src/database/schema.rs` (schema automatically created through diesel_cli) used in database, `src/database/model.rs` (model defined to be used in backend using schema) Defines the `operations between the database and the server` by calling the contents defined in and connecting to the database.
    4. `src/routes/user.rs` - You can define all routes in main.rs, but the code complexity seems to be high, so only the welcome routes are set, and the `request path and controller are connected` as routes divided for user CRUD use.
    5. `src/controllers/*` - There are files that define the `server operation for the request path`, and you can check `the types of each operation` here. Also, the entire server operation can be defined in the files in the controllers, but in order to see only the definition of each operation, it is shortened as much as possible and the `actual operation is defined in the src/services/*` folder.
    6. `src/services:*` - Actions for request paths are defined in src/controllers/*. At this time, `all actions are defined separately` in the folder that explains the increase in operation scale.
    7. `src/types/*` - A folder in which `all types (request, response, struct, ...)` are defined during server operation.

## Conclusion

1. The first rust's `ownership` and the difficulty to understand the `structure (struct, lifetime, ...)` is high.
2. It is not difficult to write simple logic.
3. Module management using `cargo is very simple and useful`. (It seems to be the best module management I have seen so far)
4. If the proficiency of the language increases, it seems that development will be possible very quickly.
5. Application build time, communication time, and logic operation time are all `very fast and efficient`.