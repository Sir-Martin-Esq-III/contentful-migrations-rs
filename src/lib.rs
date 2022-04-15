pub mod contentful_migration {
    use reqwest;
    use serde_json::json;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct client {
        pub access_token: String,
        pub space_id: Option<String>,
        pub envionment: Option<String>,
    }

    impl client {
        pub fn new(access_token: &str) -> Self {
            Self {
                access_token: access_token.to_string(),
                space_id: None,
                envionment: None,
            }
        }
        pub fn print_token(self) -> Self {
            println!("{:?}", self);
            self
        }
        pub fn space(mut self, space_id: &str) -> Self {
            self.space_id = Some(space_id.to_string());
            self
        }
        pub fn environment(mut self, environment_id: &str) -> Self {
            self.envionment = Some(environment_id.to_string());
            self
        }

        pub async fn send_req(self) -> Result<(), Box<dyn std::error::Error>> {
            let url = "https://api.contentful.com/spaces/35m8orkiglvo/environments";
            let reqwest_client = reqwest::Client::new();
            let body = json!({"name":"TOMTEST"});
            let resp = reqwest_client
                .post(url)
                .bearer_auth(&self.access_token)
                .header(
                    "Content-Type",
                    "application/vnd.contentful.management.v1+json",
                )
                .json(&body)
                .send()
                .await?;
            println!("{:?}", resp.status());
            println!("{:?}", resp.text().await?);

            Ok(())
        }
    }
}
