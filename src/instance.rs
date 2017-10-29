use client::{Client, GoogleAPIResponse};
use std::error::Error;



#[derive(Deserialize, Debug)]
pub enum InstanceStatus {
    PROVISIONING,
    STAGING,
    RUNNING,
    STOPPING,
    STOPPED,
    SUSPENDING,
    SUSPENDED,
    TERMINATED,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstanceItem {
    pub id: String,
    pub name: String,
    pub status: InstanceStatus,
}

impl InstanceItem {
    pub fn start(&self, client: &Client) -> Result<GoogleAPIResponse, Box<Error>> {
        client.post(&format!("/instances/{}/start", self.name), vec![])
    }

    pub fn stop(&self, client: &Client) -> Result<GoogleAPIResponse, Box<Error>> {
        client.post(&format!("/instances/{}/stop", self.name), vec![])
    }
}

pub type InstanceItems = Vec<InstanceItem>;

#[derive(Debug)]
pub struct InstanceOp<'a> {
    pub client: &'a Client<'a>,
}

impl<'a> InstanceOp<'a> {
    /// Get Google Cloud compute instance list
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use gcloud::{Client, InstanceOp};
    ///
    /// fn env_var(key: &str) -> String {
    ///     env::var(key).expect(&format!("{} environment variable is required", key))
    /// }
    ///
    /// let client = Client {
    ///     project: &env_var("PROJECT"),
    ///     zone: &env_var("ZONE"),
    ///     client_secret: &env_var("CLIENT_SECRET"),
    ///     client_id: &env_var("CLIENT_ID"),
    ///     refresh_token: &env_var("REFRESH_TOKEN"),
    /// };

    /// let instances = InstanceOp { client: &client }.list().unwrap();
    /// println!("{:#?}", instances);
    /// assert!(
    ///     instances.len() > 0,
    ///     "instances.len() > 0 ==> {} is not higher than 0",
    ///     instances.len()
    /// );
    /// ```
    pub fn list(&self) -> Result<InstanceItems, Box<Error>> {
        self.client.get("/instances", vec![]).and_then(|response| {
            match response {
                GoogleAPIResponse::GoogleAPISuccessResponse(resp) => Ok(resp.items),
                _ => Err(From::from("Invalid response from GoogleAPI")),
            }
        })
    }
}
