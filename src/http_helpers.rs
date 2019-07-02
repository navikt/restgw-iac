macro_rules! http_ok_try {
    ($input:expr) => {
        {
            let result = $input?;
            let status = result.status();
            if !status.is_success() {
                return Err(RestError::NotOk(status));
            }
            result
        }
    }
}

#[derive(Debug)]
pub enum RestError {
    NotOk(reqwest::StatusCode),
    ReqwestError(reqwest::Error),
}
