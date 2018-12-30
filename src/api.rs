pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub struct ApiConfig {
    pub endpoint: String,
    pub credentials: Option<Credentials>,
    pub url_params: Option<Vec<String>>,
    pub headers: Option<Vec<String>>,
    pub content_type: Option<String>,
    pub body: Option<String>,
    pub status_code: Option<u32>,
    pub risk_max: Option<u32>,
    pub risk_min: Option<u32>,
}

impl Credentials {
    pub fn from(username: String, password: String) -> Credentials {
        Credentials { username, password }
    }
}

impl ApiConfig {
    pub fn from(endpoint: String) -> ApiConfig {
        ApiConfig {
            endpoint,
            credentials: None,
            url_params: None,
            headers: None,
            content_type: None,
            body: None,
            status_code: None,
            risk_min: None,
            risk_max: None,
        }
    }
}
