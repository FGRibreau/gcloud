use client::{Client, GoogleAPIResponse};
use std::error::Error;


#[derive(Debug, Clone)]
pub struct InstanceGroupManagerOp<'a> {
    pub name: &'static str,
    pub client: &'a Client<'a>,
}


impl<'a> InstanceGroupManagerOp<'a> {
    /// Get Google Cloud compute instance list
    pub fn resize(&self, new_size: u8) -> Result<GoogleAPIResponse, Box<Error>> {
        self.client.post(
            &format!("/instanceGroupManagers/{}/resize", self.name),
            vec![("size", &new_size.to_string())],
        )
    }
}
