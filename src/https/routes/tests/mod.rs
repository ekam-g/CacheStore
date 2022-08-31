#![cfg(test)]

mod test {
    use crate::func::http_request;
    use core::time;
    use std::thread;
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
    }
    #[tokio::test]
    async fn read_data_test() {
        thread::sleep(time::Duration::from_secs(2));
        test_url("http://localhost:8000/read/test`worked`data/your_api_key".to_string()).await;
    }
    #[tokio::test]
    async fn delete_data_test() {
        thread::sleep(time::Duration::from_secs(3));
        test_url("http://localhost:8000/delete/test`worked`data.txt//your_api_key".to_string())
            .await;
    }
}
