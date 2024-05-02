use serde::Deserialize;

#[derive(Deserialize)]
pub struct TurnstileForm {
    pub username: String,
    pub password: String,
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
}


#[derive(Deserialize)]
pub struct RecaptchaV2Form {
    pub username: String,
    pub password: String,
    #[serde(rename = "g-recaptcha-response")]
    pub g_recaptcha_response: String,
}

#[derive(Deserialize)]
pub struct RecaptchaV3Form {
    pub username: String,
    pub password: String,
    #[serde(rename = "g-recaptcha-response")]
    pub recaptcha_token: String,
}

#[derive(Deserialize)]
pub struct RecaptchaEnterpriseForm {
    pub username: String,
    pub password: String,
    #[serde(rename = "recaptcha-token")]
    pub recaptcha_token: String,
}

#[derive(Deserialize)]
pub struct CaptchaFoxForm {
    pub username: String,
    pub password: String,
    #[serde(rename = "cf-captcha-response")]
    pub cf_captcha_response: String,
}