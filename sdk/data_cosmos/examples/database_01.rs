use azure_data_cosmos::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    // We expect access keys (ie, not resource constrained)
    let primary_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::primary_from_base64(&primary_key)?;

    let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());

    let database = client.database_client("pollo");
    println!("database_name == {}", database.database_name());

    let collections = database
        .list_collections()
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!("collections == {:#?}", collections);

    let collection = database
        .collection_client("cnt")
        .get_collection()
        .into_future()
        .await?;
    println!("collection == {:#?}", collection);

    Ok(())
}
