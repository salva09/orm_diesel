sudo apt-get install sqlite3 libsqlite3-dev -y

cargo install diesel_cli --no-default-features --features sqlite

diesel setup

diesel migration run
diesel migration redo

cargo build
cargo build --bin select_all
cargo build --bin insert_user