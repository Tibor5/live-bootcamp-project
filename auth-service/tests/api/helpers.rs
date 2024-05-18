use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let http_client = reqwest::Client::new();
        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_signup(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/plugin", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_login(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_logout(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_verify_token(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}