use std::io::{Error, ErrorKind};

pub const OK: StatusCode = StatusCode {
    code: 200,
    reason: "OK",
};

pub const NOT_FOUND: StatusCode = StatusCode {
    code: 404,
    reason: "NOT FOUND",
};

#[derive(Copy, Clone, Debug)]
pub struct StatusCode {
    pub code: u32,
    pub reason: &'static str,
}

const CODE_TO_STATUS: [(u32, StatusCode); 2] = [(NOT_FOUND.code, NOT_FOUND), (OK.code, OK)];

impl TryFrom<u32> for StatusCode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(CODE_TO_STATUS
            .iter()
            .find(|s| s.0 == value)
            .ok_or(ErrorKind::InvalidInput)?
            .1)
    }
}
