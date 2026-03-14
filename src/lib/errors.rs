use dotenvy::Error as DotEnvError;
use regex::Error as RegexError;
use reqwest::{
    header::ToStrError, Error as ReqwestError
};
use serde_json::Error as SerdeError;
use std::{env::VarError, fmt::Display, io::Error as IoError, net::TcpStream, num::ParseIntError};
use native_tls::{Error as NativeTlsError, HandshakeError};

#[derive(Debug)]
pub enum Errors {
    Reqwest(ReqwestError),
    DotEnv(DotEnvError),
    Deserialize(SerdeError),
    Regex(RegexError),
    EnvVar(VarError),
    Io(IoError),
    NativeTls(NativeTlsError),
    Handshake(HandshakeError<TcpStream>),
    ToStr(ToStrError),
    ParseInt(ParseIntError),
}

impl From<ReqwestError> for Errors {
    fn from(error: ReqwestError) -> Self {
        Self::Reqwest(error)
    }
}

impl From<DotEnvError> for Errors {
    fn from(error: DotEnvError) -> Self {
        Self::DotEnv(error)
    }
}

impl From<VarError> for Errors {
    fn from(error: VarError) -> Self {
        Self::EnvVar(error)
    }
}

impl From<RegexError> for Errors {
    fn from(error: RegexError) -> Self {
        Self::Regex(error)
    }
}

impl From<SerdeError> for Errors {
    fn from(error: SerdeError) -> Self {
        Self::Deserialize(error)
    }
}

impl From<IoError> for Errors {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<NativeTlsError> for Errors {
    fn from(value: NativeTlsError) -> Self {
        Self::NativeTls(value)
    }
}

impl From<HandshakeError<TcpStream>> for Errors {
    fn from(value: HandshakeError<TcpStream>) -> Self {
        Self::Handshake(value)
    }
}

impl From<ToStrError> for Errors {
    fn from(value: ToStrError) -> Self {
        Self::ToStr(value)
    }
}

impl From<ParseIntError> for Errors {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(error) => write!(f, "Unable to send request => {error}"),
            Self::DotEnv(error) => write!(f, "Unable to load environment file => {error}"),
            Self::Deserialize(error) => write!(f, "Could not deserialize bytes => {error}"),
            Self::Regex(error) => write!(f, "Unable to build regular expression => {error}"),
            Self::EnvVar(error) => write!(f, "Could not load environment variable => {error}"),
            Self::Io(error) => write!(f, "Unable to create file due to error => {error}"),
            Self::NativeTls(error) => write!(f, "TLS error => {error}"),
            Self::Handshake(error) => write!(f, "Unable to successfully complete TCP handshake => {error}"),
            Self::ToStr(error) => write!(f, "Unable to convert to str => {error}"),
            Self::ParseInt(error) => write!(f, "Could not parse integer => {error}"),
        }
    }
}
