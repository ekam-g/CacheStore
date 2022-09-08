# Welcome To Rust Database!

# what is this?

this is a database for you to git clone and use in your project!

# why should i use it?

It is fast and it takes 2ms to return data. Also if you want to local store data on your pc of your users then you may want to use it.

# How should i use it?

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

=> GET / (index)
=> GET /read/<path>/<api_key> (read)
=> GET /add/<path>/<data_name>/<data>/<api_key> (add)
=> GET /delete/<path>/<api_key> (delete)
=> GET /null_write/<path>/<api_key> (null_write)

## How can you use it?

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

# How Does this database write data
