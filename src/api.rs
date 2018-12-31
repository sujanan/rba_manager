pub struct Endpoint {
    value: String,
}

pub struct Credentials {
    username: String,
    password: String,
}

pub struct Headers {
    value: Vec<String>,
}

pub struct ContentType {
    value: String,
}

pub struct Body {
    value: String,
}

pub struct StatusCode {
    value: u32,
}

pub struct Risk {
    min: u32,
    max: u32,
}

pub struct ApiConfig {
    name: String,
    endpoint: Option<Endpoint>,
    credentials: Option<Credentials>,
    headers: Option<Headers>,
    content_type: Option<ContentType>,
    body: Option<Body>,
    status_code: Option<StatusCode>,
    risk: Option<Risk>,
}

trait Validator {
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

trait ApiConfigBuilder {
    fn drown(self, a: &mut ApiConfig);
}

impl Validator for Endpoint {}
impl Validator for Credentials {}
impl Validator for Headers {}
impl Validator for ContentType {}
impl Validator for Body {}
impl Validator for StatusCode {}
impl Validator for Risk {}

impl ApiConfigBuilder for Endpoint {
    fn drown(self, a: &mut ApiConfig) {
        a.endpoint = Some(self);
    }
}

impl Credentials {
    pub fn new(username: String, password: String) -> Credentials {
        Credentials { username, password }
    }
}

impl ApiConfig {
    pub fn new(name: String) -> ApiConfig {
        ApiConfig {
            name,
            endpoint: None,
            credentials: None,
            headers: None,
            content_type: None,
            body: None,
            status_code: None,
            risk: None,
        }
    }

    pub fn add<F, T: Validator + ApiConfigBuilder>(&mut self, func: &F) -> &mut ApiConfig
    where
        F: Fn() -> T,
    {
        func().drown(self);
        self
    }
}
