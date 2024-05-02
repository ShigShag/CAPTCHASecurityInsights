use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecaptchaEnterpriseForm {
    pub username: String,
    pub password: String,
    #[serde(rename = "recaptcha-token")]
    pub recaptcha_token: String,
}
