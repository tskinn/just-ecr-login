use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecr::{Client, Error};
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env()
        .behavior_version(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);
    let resp = &client.get_authorization_token().send().await?.authorization_data.unwrap()[0];
    let token = resp.authorization_token.clone().unwrap();
    let token = &general_purpose::STANDARD.decode(token).unwrap();
    let token = &token[4..];
    println!("{}", std::str::from_utf8(token).unwrap());
    Ok(())
}
