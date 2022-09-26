use crate::StateData;

// Local functions testing
//_________________________________________________________________________________________________________________________
fn test_read_error(func: StateData, what_error: String) {
    let check_data = func.read_data("test/worked/local/data");
    match check_data {
        Ok(data) => {
            panic!("{}", data[0].to_owned())
        }
        Err(e) => {
            if e != what_error {
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn local_func_test() {
    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.write_data("this is going very well", "test/worked/local", "data")
        .expect("failed when writing data");
    func.write_data("this is going very well", "test/worked/local", "data")
        .expect("failed when writing data");

    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    let check_data = func
        .read_data("test/worked/local/data")
        .expect("failed when reading");
    if check_data
        != vec![
        "this is going very well".to_owned(),
        "this is going very well".to_owned(),
    ]
    {
        panic!("write data did not work!")
    }


    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.null_write("test/worked/local/data")
        .expect("write null functions failed");
    func.write_data("test/worked/local", "test/worked/local", "data")
        .expect("msg");
    test_read_error(func, "data is null".to_owned());

    let func = StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    };
    func.delete_data("test/worked/local/data")
        .expect("delete failed");
    test_read_error(func, "No such file or directory (os error 2)".to_owned());
}
