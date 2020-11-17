sudo apt-get install sqlite3 libsqlite3-dev -y

cargo install diesel_cli --no-default-features --features sqlite

echo "DATABASE_URL=db.sqlite3" > .env

diesel setup

diesel migration run
diesel migration redo

cargo build --release
cargo build --bin select_all --release
cargo build --bin insert_user --release
cargo build --bin delete_user --release
cargo build --bin query --release
