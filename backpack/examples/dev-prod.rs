use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber with env filter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    env_rs::init()?;

    // Now you can access environment variables
    if let Ok(app_env) = env::var("APP_ENV") {
        println!("APP_ENV: {app_env}");
    } else {
        println!("APP_ENV not set");
    }

    if let Ok(test) = env::var("TEST") {
        println!("TEST: {test}");
    } else {
        println!("TEST not set");
    }

    Ok(())
}
