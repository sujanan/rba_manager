#[derive(Debug)]
pub struct Endpoint {
    pub value: String,
}

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Headers {
    pub value: Vec<String>,
}

#[derive(Debug)]
pub struct ContentType {
    pub value: String,
}

#[derive(Debug)]
pub struct Body {
    pub value: String,
}

#[derive(Debug)]
pub struct StatusCode {
    pub value: u32,
}

#[derive(Debug)]
pub struct Risk {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug)]
pub struct ApiConfig {
    endpoint: Option<Endpoint>,
    credentials: Option<Credentials>,
    headers: Option<Headers>,
    content_type: Option<ContentType>,
    body: Option<Body>,
    status_code: Option<StatusCode>,
    risk: Option<Risk>,
}

pub trait Validator {
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

pub trait ApiConfigBuilder {
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

impl ApiConfigBuilder for Credentials {
    fn drown(self, a: &mut ApiConfig) {
        a.credentials = Some(self);
    }
}

impl ApiConfigBuilder for Headers {
    fn drown(self, a: &mut ApiConfig) {
        a.headers = Some(self);
    }
}

impl ApiConfigBuilder for Body {
    fn drown(self, a: &mut ApiConfig) {
        a.body = Some(self);
    }
}

impl ApiConfigBuilder for StatusCode {
    fn drown(self, a: &mut ApiConfig) {
        a.status_code = Some(self);
    }
}

impl ApiConfigBuilder for Risk {
    fn drown(self, a: &mut ApiConfig) {
        a.risk = Some(self);
    }
}

impl ApiConfig {
    pub fn new() -> ApiConfig {
        ApiConfig {
            endpoint: None,
            credentials: None,
            headers: None,
            content_type: None,
            body: None,
            status_code: None,
            risk: None,
        }
    }

    pub fn add<F, T: Validator + ApiConfigBuilder>(&mut self, func: F) -> &mut ApiConfig
    where
        F: Fn() -> T,
    {
        let rv = func();
        rv.validate().map_err(|error| {
            panic!("error: 'ApiConfig' failed to invalid property: {}", error);
        });
        rv.drown(self);
        self
    }
}
