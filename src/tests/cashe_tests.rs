use crate::func::http_request;
use crate::tests::{BASE_URL, test_url};

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
