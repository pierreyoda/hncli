use api::HnClient;
use errors::HnCliError;

mod api;
mod errors;

#[tokio::main]
async fn main() -> Result<(), HnCliError> {
    let client = HnClient::new()?;

    let test = client.get_item(1).await?;
    dbg!(test);

    Ok(())
}
