# Welcome To CacheStore Database!


Note the name was changed to cache_store so use this https://crates.io/crates/cache_store

## What is this?

this is a cache database for you to git clone and use in your project!

## Why should I use it?

It is fast, and it takes 2ms to return data. Also, if you want to local store data on your pc of your users then you may want to use it.

## Start the online database

```
use cache_store;

fn main() {
    cache_store::StateData {
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
    => GET /read/<path>/<api_key> (read_ssd)
    => GET /add/<path>/<data_name>/<data>/<api_key> (add_ssd)
    => GET /delete/<path>/<api_key> (delete)
    => GET /null_write/<path>/<api_key> (null_write_sdd)
    => GET /add_key/<key>/<data>/<api_key> (add_ram)
    => GET /read_key/<key>/<api_key> (read_ram)
    => GET /delete_key/<key>/<api_key> (delete_ram)
  ```

## How can you use it?

USE RUST NIGHTLY!!!!!!!

use ` instead of / to navigate directories.

This request will add data at ${your_database_folder}/test/worked data.txt. The data.txt will have "this is going very well" stored.

```
{}/add/test`worked/data/this is going very well/your_api_key
```

This request will delete at ${your_database_folder}/test/worked data.txt.

```
{}/delete/test`worked`data/your_api_key
```

This request will read data at ${your_database_folder}/test/worked data.txt.

```
{}/read/test`worked`data/your_api_key
```

This request will write null at ${your_database_folder}/test/worked data.txt.

```
{}/null_write/test`worked`data/your_api_key
```

This request will cache hello at key worked in your ram.

```
{}/add_key/worked/hello/your_api_key
```

This request read cached worked in your ram.

```
{}/read_key/worked/your_api_key
```

This request will delete cache worked in your ram.

```
{}/delete_key/worked/hello/your_api_key
```

Info on api

```
{}/
```

## Local Host Functions

To use this you don't need to start the database.

To write

```
use cache_store;

fn main(){
    let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        func.write_data("this is going very well", "test/worked/local", "data")
            .expect("failed when writing data");
}
```

To read

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        let check_data = func
            .read_data("test/worked/local/data")
            .expect("failed when reading");
}
```

To Delete

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        func.delete_data("test/worked/local/data")
            .expect("delete failed");
}
```

To write null

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        func.null_write("test/worked/local/data")
            .expect("write null functions failed");
}
```

To write hello cache in worked

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
            func.add_cache_data("hello", "worked").expect("");
}
```

To delete hello cache

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
            func.delete_cache_data("hello").expect("");
}
```

To read hello cache

```
use cache_store;

fn main(){
     let func = cache_store::StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
            func.read_cache_data("hello").expect("");
}
```


# Questions?

## How Does this database write data?

This database uses txt instead of csv or json because txt is the fastest way to read and write data.
It also uses a ram key storage too!

## How Does it organize data?

This database uses a directory way just like you would do on a computer. This storage method automatically make files for you too!

## How Does it read data?

This database requires you to put a directory (if you want to use a "/" use a "`" instead) and it returns txt value by reading it line by line (a vec value).

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

otherwise it shows the error it received like this.

```
{
    "error" : "error when writing data"
}
```

For local functions is returns a Result<(), String> and the errors are the same just make sure to handle the error by matching the string.

## Still have questions?

Dm me in discord at Carghai88#1468

## Pull Requests?

I will respond as fast as possible just make sure it follows my ideology of speed!

