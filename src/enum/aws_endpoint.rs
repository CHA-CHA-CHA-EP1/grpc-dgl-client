#[derive(Debug)]
pub enum ApiEndpoint {
    ApplicationToken,
}

impl ApiEndpoint {
    pub fn path(&self) -> &'static str {
        match self {
            Self::ApplicationToken => "/api/v1/application/token",
        }
    }
}
