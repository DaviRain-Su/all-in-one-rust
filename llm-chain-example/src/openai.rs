use async_openai::Client;
use std::error::Error;

#[tokio::test]
async fn test_model_list() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let model_list = client.models().list().await?;

    println!("List of models:\n {:#?}", model_list.data);

    let model = client.models().retrieve("text-davinci-003").await?;
    println!("text-davinci-003 model id: {}", model.id);

    Ok(())
}
