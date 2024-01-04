use openapi::apis::configuration;
use openapi::apis::instance_api;

#[tokio::main]
async fn main() {
    let config = configuration::Configuration {
        bearer_access_token: Some("secret".to_string()),
        ..Default::default()
    };
    
    let res = instance_api::instance_get(&config).await;

    match res {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(error) => eprintln!("Error fetching instances: {}", error),
    };
}
