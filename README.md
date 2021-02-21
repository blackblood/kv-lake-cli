# kv-lake-cli
A simple client program to interact with the kv-lake server

Basic usage:
1. Just clone this repo
2. cd to the repo
3. run `cargo run`

By default it tries to connect to the server at port 8000. You can pass the port number your server is running on as an argument to `cargo run`.
eg: `cargo run 4000`
You should see the following output:
```
your-user-name:kv-lake-cli username$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/kv-lake-cli`
connected to server at port: 8000
```
You can now enter commands like below
```
PUT first_name john
PUT last_name doe
PUT age 28
GET first_name
GET age
DEL last_name
```
Note: Right now, only PUT, GET and DEL commands are supported.
