# Simple project using Diesel ORM

## Setup all and build
There is a script called ```build.sh``` that will install all the necessary and build the project (give it some time). \
The script runs on Debian-based and Arch-based distros.

## Run the program
In order to run the programs, run the following commands.
```bash
# Print out all the users in the database
target/release/select_all

target/release/insert_user

target/release/delete_user

# this one print all users with and age of 18
target/release/query
```

## Built With

* [Rust](https://www.rust-lang.org/) - Programming language
* [SQLite](https://www.sqlite.org/index.html) - SQL database engine
* [Diesel](http://diesel.rs/) - ORM Framework

## Author

* **Salvador** - [salva09](https://github.com/salva09)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE) file for details
