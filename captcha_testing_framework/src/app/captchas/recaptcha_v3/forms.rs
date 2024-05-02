use serde::Deserialize;

#[derive(Deserialize)]
pub struct RecaptchaV3Form {
    pub username: String,
    pub password: String,
    #[serde(rename = "g-recaptcha-response")]
    pub recaptcha_token: String,
}
