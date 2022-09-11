#![cfg(test)]

mod test {
    use crate::func::http_request;
    use crate::StateData;
    use core::time;
    use std::thread::{self};
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
            "http://localhost:8000/add/test`worked/data/this is going very well/your_api_key"
                .to_string(),
        )
        .await;
        test_url(
            "http://localhost:8000/add/test`worked/data/this is going very well/your_api_key"
                .to_string(),
        )
        .await;
    }
    #[tokio::test]
    async fn read_data_test() {
        thread::sleep(time::Duration::from_secs(1));
        test_url("http://localhost:8000/read/test`worked`data/your_api_key".to_string()).await;
    }
    #[tokio::test]
    async fn read_data_test_many() {
        thread::sleep(time::Duration::from_secs(1));
        let data = http_request::Request::read_more(
            "http://localhost:8000/read/test`worked`data/your_api_key".to_string(),
        )
        .await
        .expect("");
        if data.data
            != vec![
                "this is going very well".to_string(),
                "this is going very well".to_string(),
            ]
        {
            panic!("data does not equal correct amount")
        }
    }
    #[tokio::test]
    async fn delete_data_test() {
        thread::sleep(time::Duration::from_secs(3));
        test_url("http://localhost:8000/delete/test`worked`data.txt/your_api_key".to_string())
            .await;
    }
    #[tokio::test]
    async fn delete_data_test_check() {
        thread::sleep(time::Duration::from_secs(4));
        let data = http_request::Request::read(
            "http://localhost:8000/read/test`worked`data/your_api_key".to_string(),
        )
        .await
        .expect("");
        if data.error == "Success" {
            panic!("data was not deleted")
        }
    }
    #[tokio::test]
    async fn null_test() {
        thread::sleep(time::Duration::from_secs(5));
        test_url("http://localhost:8000/null_write/test`worked`data.txt/your_api_key".to_string())
            .await;
    }
    #[tokio::test]
    async fn null_test_check() {
        thread::sleep(time::Duration::from_secs(6));
        test_url(
            "http://localhost:8000/add/test`worked/data/this is going very well/your_api_key"
                .to_string(),
        )
        .await;
    }
    #[tokio::test]
    async fn read_data_test_null() {
        thread::sleep(time::Duration::from_secs(7));
        test_url("http://localhost:8000/read/test`worked`data/your_api_key".to_string()).await;
        test_url("http://localhost:8000/delete/test`worked`data.txt/your_api_key".to_string())
            .await;
    }
    // Local funtion testing
    #[tokio::test]
    async fn add_data_test_local() {
        let func = StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        func.write_data("this is going very well", "test/worked/local", "data")
            .expect("failed when writing data");
        func.write_data("this is going very well", "test/worked/local", "data")
            .expect("failed when writing data");
    }
    #[tokio::test]
    async fn read_data_test_local() {
        thread::sleep(time::Duration::from_secs(1));
        let func = StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        let check_data = func
            .read_data("test/worked/local/data.txt")
            .expect("failed when reading");
        if check_data
            != vec![
                "this is going very well".to_string(),
                "this is going very well".to_string(),
            ]
        {
            panic!("write data did not work!")
        }
    }
    #[tokio::test]
    async fn null_test_check_local() {
        thread::sleep(time::Duration::from_secs(2));
        let func = StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        let check_data = func.read_data("test/worked/local/data.txt");
        match check_data {
            Ok(_) => {
                panic!("null write failed!")
            }
            Err(e) => {
                if e != "data is null" {
                    panic!("wrong error")
                }
            }
        }
    }
    #[tokio::test]
    async fn delete_data_test_check_local() {
        thread::sleep(time::Duration::from_secs(3));
        let func = StateData {
            api_key: "your_api_key".to_string(),
            null: "null_nil_value_key:345n,234lj52".to_string(),
            data_storage_location: "database/".to_string(),
        };
        func.delete_data("test/worked/local/data.txt")
            .expect("delete failed");
    }
}
