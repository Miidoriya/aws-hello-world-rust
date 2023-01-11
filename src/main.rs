use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

/// Lists your DynamoDB Tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
