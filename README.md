## Local development
Make sure you have a local mysql server running on your machine. If you use Windows, I recommend [XAMPP](https://www.apachefriends.org/download.html). Otherwise, install docker, and in the root directory, run `docker-compose up -d`.

Make sure you've run `cargo install diesel_cli` to install the Diesel CLI, then run `diesel migration run` to set up the database.

Then run `cargo run` to start the server.
