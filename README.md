# Welcome To Rust Database!

## What is this?

this is a database for you to git clone and use in your project!

## Why should i use it?

It is fast and it takes 2ms to return data. Also if you want to local store data on your pc of your users then you may want to use it.

## Start the online database

```
use rust_store;

fn main() {
    rust_store::StateData {
        api_key: "your_api_key".to_string(),
        // make sure to change this to some key!
        // change this if your want but otherwise don't
        null: "null_nil_value_key:345n,234lj52".to_string(),
        // make sure this file exists or the code will not work!
        data_storage_location: "database/".to_string(),
    }
    .start_database_online()
}

```

## What requests can you do?

```
=> GET / (index)
=> GET /read/<path>/<api_key> (read)
=> GET /add/<path>/<data_name>/<data>/<api_key> (add)
=> GET /delete/<path>/<api_key> (delete)
=> GET /null_write/<path>/<api_key> (null_write)
```

## How can you use it?

USE RUST NIGHTLY!!!!!!!

use ` instead of / to navgate directories.

This request will add data at ${your_database_folder}/test/worked data.txt. The data.txt will have "this is going very well" stored.

```
http://localhost:8000/add/test`worked/data/this is going very well/your_api_key
```

This request will delete at ${your_database_folder}/test/worked data.txt.

```
http://localhost:8000/delete/test`worked`data.txt/your_api_key
```

This request will read data at ${your_database_folder}/test/worked data.txt.

```
http://localhost:8000/read/test`worked`data/your_api_key
```

This request will write null at ${your_database_folder}/test/worked data.txt.

```
http://localhost:8000/null_write/test`worked`data.txt/your_api_key
```

Info on api

```
http://localhost:8000/
```

# Questions?

## How Does this database write data?

This database uses txt instead of csv or json because txt is the fastest way to read and write data.

## How Does it organize data?

This database uses a directory way just like you would do on a computer. This storage method automatically make files for you too!

## How Does it read data?

This database requires your to put a directory (if you want to use  a "/" use a "`" instead) and it return txt value by reading it line by line (a vec value). 

## How does this database value handle null?

This database has a write null feature to write a null key. If the null key is read when your read a txt it returns

```
{
    "data" : "null"
    "error" : "data is null"
}
```
make sure to handle error and check if error value equals "Success".

## Error Handling? 

If the database work just as planned it returns
```
{
    "error" : "Success"
}
```
otherwise it shows the error it received. 

## Still have questions?

Dm me in discord at Carghai88#1468

## Pull Requests?

I will respond as fast as possible just make sure it follows my ideology of speed!
