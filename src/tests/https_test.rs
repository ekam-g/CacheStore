use crate::func::http_request;
use crate::tests::{BASE_URL, test_url};

#[tokio::test]
async fn data_test() {
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
