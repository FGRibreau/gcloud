

use tap::TapResultOps;

use reqwest;
use reqwest::Method;
use querystring;
use instance::{InstanceItems, empty_instance_items};

use std::error::Error;
use querystring::QueryParams;
use std::collections::HashMap;

/// Google Cloud OAuth2 Token endpoint
const TOKEN_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v4/token";


#[derive(Deserialize, Debug, Clone)]
pub struct GoogleAPISuccessResponse {
    pub kind: String,
    pub id: String,

    #[serde(default = "empty_instance_items")]
    pub items: InstanceItems,
}


#[derive(Deserialize, Debug, Clone)]
pub enum GoogleAPIComputeOperationStatus {
    PROGRESS,
    DONE,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAPISuccessComputeOperationResponse {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub zone: String,
    pub operation_type: String,
    pub status: GoogleAPIComputeOperationStatus,
    pub user: String,
    pub progress: u8,
    pub insert_time: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleAuthAPISuccessResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleAuthAPIErrorResponse {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleAPIErrorResponse {
    pub error: GoogleAPIError,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleAPIError {
    pub code: u64,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GoogleAPIResponse {
    GoogleAPISuccessComputeOperationResponse(GoogleAPISuccessComputeOperationResponse),
    GoogleAPISuccessResponse(GoogleAPISuccessResponse),
    GoogleAuthAPISuccessResponse(GoogleAuthAPISuccessResponse),
    GoogleAuthAPIErrorResponse(GoogleAuthAPIErrorResponse),
    GoogleAPIErrorResponse(GoogleAPIErrorResponse),
}

#[derive(Debug, Clone)]
pub struct Client<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub refresh_token: &'a str,

    /// Google Cloud project name, e.g. `myproject-181100`
    pub project: &'a str,

    /// Google Cloud zone, e.g. `europe-west1-d`
    pub zone: &'a str,
}


impl<'a> Client<'a> {
    /// Ask for a new access_token using the refresh_token
    fn refresh_token(&self) -> Result<GoogleAPIResponse, Box<Error>> {
        let mut params = HashMap::new();

        params.insert("client_id", self.client_id);
        params.insert("client_secret", self.client_secret);
        params.insert("refresh_token", self.refresh_token);
        params.insert("grant_type", "refresh_token");

        Ok(reqwest::Client::new()
            .post(TOKEN_ENDPOINT)
            .form(&params)
            .send()?
            .json()?)
    }

    /// Get a new `access_token`, it will use the `refresh_token` underneath
    /// @todo only ask for a new access_token when it's too old
    fn get_access_token(&self) -> String {
        match self.refresh_token() {
            Ok(GoogleAPIResponse::GoogleAuthAPISuccessResponse(resp)) => {
                trace!("{:#?}", resp);
                resp.access_token
            }
            _ => String::new(),
        }
    }

    fn url(&self, path: &str, query: QueryParams) -> String {
        let access_token = self.get_access_token();

        let query_string = querystring::stringify(
            [
                &query[..],
                &vec![("access_token", access_token.as_str())][..],
            ].concat(),
        );

        let url = format!(
            "https://www.googleapis.com/compute/v1/projects/{}/zones/{}{}?{}",
            self.project,
            self.zone,
            path,
            query_string
        );
        debug!("url={}", url);
        url
    }

    /// Run a Get HTTP query
    pub fn get(&self, path: &str, query: QueryParams) -> Result<GoogleAPIResponse, Box<Error>> {
        let client = reqwest::Client::new();

        Ok(client
            .request(Method::Get, &self.url(path, query))
            .send()
            .tap_ok(|resp| println!("Response: {:#?}", resp))?
            .json()?)
    }

    /// Run a Post HTTP query
    pub fn post(&self, path: &str, query: QueryParams) -> Result<GoogleAPIResponse, Box<Error>> {
        let client = reqwest::Client::new();

        Ok(client
            .request(Method::Post, &self.url(path, query))
            .body("")
            .send()
            .tap_ok(|resp| println!("Response: {:#?}", resp))?
            .json()?)
    }
}
