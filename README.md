# Simple project using Diesel ORM

## Setup all and build

There is a script called `build.sh` that will install all the necessary and build the project (give it some time). \
The script runs on Debian-based and Arch-based distros.

## Run the program

In order to run the programs, run the following commands.

```bash
# Print out all the users in the database
target/release/select_all

target/release/insert_user

target/release/delete_user

# Print all users with an age of 18
target/release/query
```

## Built With

- [Rust](https://www.rust-lang.org/) - Programming language
- [SQLite](https://www.sqlite.org/index.html) - SQL database engine
- [Diesel](http://diesel.rs/) - ORM Framework

## Authors

- **Salvador** - [salva09](https://github.com/salva09)
- **Yael** - [Yael2407](https://github.com/Yael2407)
- **Silvia** - [layefly](https://github.com/layefly)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE) file for details
