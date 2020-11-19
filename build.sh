# Install dependencies (sqlite and dev libraries)
if [ -f "/etc/debian-version" ]; then
    sudo apt-get install sqlite3 libsqlite3-dev -y
elif [ -f "/etc/arch-release" ]; then
    sudo pacman -S sqlite --noconfirm
fi

# Install diesel cli tool
cargo install diesel_cli --no-default-features --features sqlite

# Create .env file for environment variables
echo "DATABASE_URL=db.sqlite3" > .env

# API setup and database creation
diesel setup
diesel migration run
diesel migration redo

# Application building
cargo build --release
cargo build --bin select_all --release
cargo build --bin insert_user --release
cargo build --bin delete_user --release
cargo build --bin query --release
