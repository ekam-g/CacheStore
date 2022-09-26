#![cfg(test)]


use crate::func::http_request;
use crate::StateData;

static BASE_URL: &str = "http://localhost:5000";
// http://0.0.0.0:5000

async fn test_url(url: String) {
    let test_data = http_request::Request::read(url).await;
    match test_data {
        Err(e) => {
            panic!("Make Sure Main Code is running\n\n {}", e)
        }
        Ok(data) => {
            if data.error != "Success" {
                panic!("{}", data.error);
            }
        }
    }
}

#[tokio::test]
async fn add_data_test() {
    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;
    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;

    test_url(format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned()).await;

    let data = http_request::Request::read_more(
        format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned(),
    )
        .await
        .expect("");
    if data.data
        != vec![
        "this is going very well".to_owned(),
        "this is going very well".to_owned(),
    ]
    {
        panic!("data does not equal correct amount")
    }

    test_url(format!("{}/delete/test`worked`data/your_api_key", BASE_URL).to_owned()).await;

    let data = http_request::Request::read(
        format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned(),
    )
        .await
        .expect("");
    if data.error == "Success" {
        panic!("data was not deleted")
    }


    test_url(format!("{}/null_write/test`worked`data/your_api_key", BASE_URL).to_owned()).await;


    test_url(
        format!(
            "{}/add/test`worked/data/this is going very well/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;

    test_url(format!("{}/read/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
    test_url(format!("{}/delete/test`worked`data/your_api_key", BASE_URL).to_owned()).await;
}

//Cashe Testing
//___________________________________________________________________________________________________________
#[tokio::test]
async fn data_test_cashe() {
    test_url(
        format!(
            "{}/add_key/worked/hello/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;

    test_url(
        format!(
            "{}/read_key/worked/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;


    test_url(
        format!(
            "{}/delete_key/worked/your_api_key",
            BASE_URL
        )
            .to_owned(),
    )
        .await;


    let data = http_request::Request::read(
        format!("{}/read_key/worked/your_api_key", BASE_URL).to_owned(),
    )
        .await
        .expect("");
    if data.error == "Success" {
        panic!("data was not deleted")
    }
}


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
fn local_func() {
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
