use serde::Deserialize;

#[derive(Deserialize)]
pub struct CaptchaFoxForm {
    pub username: String,
    pub password: String,
    #[serde(rename = "cf-captcha-response")]
    pub cf_captcha_response: String,
}