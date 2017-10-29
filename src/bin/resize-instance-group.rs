extern crate gcloud;

use std::env;
use gcloud::{Client, InstanceGroupManagerOp};

fn env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} environment variable is required", key))
}

fn main() {
    let client = Client {
        project: &env_var("PROJECT"),
        zone: &env_var("ZONE"),
        client_secret: &env_var("CLIENT_SECRET"),
        client_id: &env_var("CLIENT_ID"),
        refresh_token: &env_var("REFRESH_TOKEN"),
    };

    println!(
        "{:#?}",
        InstanceGroupManagerOp {
            name: "render-group",
            client: &client,
        }.resize(0)
    );
}
