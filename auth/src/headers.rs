use actix_web::http::header::{
    Header,
    HeaderName,
    HeaderValue,
    InvalidHeaderValue,
    TryIntoHeaderValue,
};
use actix_web::error::ParseError;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientSecretHeader(String);

impl ClientSecretHeader {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn inner(self) -> String {
        self.0
    }
}

impl Header for ClientSecretHeader {
    fn name() -> HeaderName {
        HeaderName::from_static("client-secret")
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let Some(header) = msg.headers().get(Self::name()) else {
            return Err(ParseError::Header);
        };
        let bytes = header.as_bytes();
        match std::str::from_utf8(bytes) {
            Ok(string) => Ok(Self(string.to_string())),
            Err(e) => Err(ParseError::Utf8(e)),
        }
    }
}

impl TryIntoHeaderValue for ClientSecretHeader {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdTokenHeader(Vec<u8>);

impl IdTokenHeader {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Header for IdTokenHeader {
    fn name() -> HeaderName {
        HeaderName::from_static("id-token")
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let Some(header) = msg.headers().get(Self::name()) else {
            return Err(ParseError::Header);
        };

        Ok(Self(header.as_bytes().to_vec()))
    }
}

impl TryIntoHeaderValue for IdTokenHeader {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_bytes(&self.0)
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientIdHeader(Uuid);

impl ClientIdHeader {
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl Header for ClientIdHeader {
    fn name() -> HeaderName {
        HeaderName::from_static("client-id")
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let Some(header) = msg.headers().get(Self::name()) else {
            return Err(ParseError::Header);
        };

        let Ok(id) = header.to_str() else {
            return Err(ParseError::Header);
        };

        let Ok(id) = uuid::Uuid::parse_str(id) else {
            return Err(ParseError::Header);
        };

        Ok(Self(id))
    }
}

impl TryIntoHeaderValue for ClientIdHeader {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.hyphenated().to_string())
    }
}
