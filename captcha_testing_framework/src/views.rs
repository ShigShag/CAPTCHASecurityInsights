use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Result};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use tera::{Context, Tera};
use serde_urlencoded;

use crate::models::*;

pub async fn landing_page(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let context = Context::new();

    let rendered = tera
        .render("index.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn turnstile(tera: web::Data<Tera>) -> Result<HttpResponse> {
    dotenv().ok();

    let mut context = Context::new();
    match std::env::var("TURNSTILE_SITE_KEY") {
        Ok(sitekey) => {
            // Insert custom sitekey
            context.insert("site_key", &sitekey);

            let rendered = tera
                .render("captchas/turnstile.html", &context)
                .map_err(|e| {
                    eprintln!("Template error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template rendering error")
                })?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Err(e) => {
            eprintln!("Environment variable error: {:?}", e); // Log or handle the env var error
            Ok(HttpResponse::InternalServerError()
                .body("Internal server error: TURNSTILE_SITE_KEY not found"))
        }
    }
}

pub async fn submit_turnstile(post_data: web::Form<TurnstileForm>) -> Result<HttpResponse> {
    // Check env
    dotenv().ok();

    // Load API key
    let secret_key = match std::env::var("TURNSTILE_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("TURNSTILE_SECRET_KEY is not set in environment."))
        }
    };

    // Send post request to api endpoint
    let client = Client::new();
    let response = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .json(&serde_json::json!({
            "secret": secret_key,
            "response": post_data.cf_turnstile_response
        }))
        .send()
        .await;

    // Handle network or request sending errors
    let response = match response {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Error sending verification request.")
            );
        }
    };

    // Parse response JSON
    let turnstile_response: Result<serde_json::Value, _> = response.json().await;

    match turnstile_response {
        Ok(data) => {
            if data.get("success").and_then(serde_json::Value::as_bool) == Some(true) {
                Ok(HttpResponse::Ok().body(format!("Form submission successful: {:?}", data)))
            } else {
                Ok(HttpResponse::BadRequest()
                    .body(format!("Captcha verification failed: {:?}", data)))
            }
        }
        Err(e) => {
            eprintln!("JSON parsing error: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Error processing verification response."))
        }
    }
}


pub async fn recaptcha_v2(tera: web::Data<Tera>) -> Result<HttpResponse> {
    dotenv().ok();

    let mut context = Context::new();
    match std::env::var("RECAPTCHA_V2_SITE_KEY") {
        Ok(sitekey) => {
            context.insert("site_key", &sitekey);

            let rendered = tera
                .render("captchas/recaptcha_v2.html", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Err(e) => {
            eprintln!("Environment variable error: {:?}", e);
            Ok(HttpResponse::InternalServerError()
                .body("Internal server error: RECAPTCHA_V2_SITE_KEY not found"))
        }
    }
}

pub async fn submit_recaptcha_v2(post_data: web::Form<RecaptchaV2Form>) -> Result<HttpResponse> {
    // Check env
    dotenv().ok();

    // Load API key
    let secret_key = match std::env::var("RECAPTCHA_V2_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("RECAPTCHA_V2_SECRET_KEY is not set in environment."))
        }
    };

    // Send post request to api endpoint
    let client = Client::new();
    let response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[
            ("secret", secret_key),
            ("response", post_data.g_recaptcha_response.clone()),
        ])
        .send()
        .await;

    // Handle network or request sending errors
    let response = match response {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Error sending verification request.")
            );
        }
    };

    // Parse response JSON
    let recaptcha_response: Result<serde_json::Value, _> = response.json().await;

    match recaptcha_response {
        Ok(data) => {
            if data.get("success").and_then(serde_json::Value::as_bool) == Some(true) {
                Ok(HttpResponse::Ok().body(format!("Form submission successful: {:?}", data)))
            } else {
                Ok(HttpResponse::BadRequest()
                    .body(format!("Captcha verification failed: {:?}", data)))
            }
        }
        Err(e) => {
            eprintln!("JSON parsing error: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Error processing verification response."))
        }
    }
}

pub async fn recaptcha_v3(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    match std::env::var("RECAPTCHA_V3_SITE_KEY") {
        Ok(sitekey) => {
            context.insert("site_key", &sitekey);

            let rendered = tera
                .render("captchas/recaptcha_v3.html", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Err(e) => {
            eprintln!("Environment variable error: {:?}", e);
            Ok(HttpResponse::InternalServerError()
                .body("Internal server error: RECAPTCHA_V3_SITE_KEY not found"))
        }
    }
}

pub async fn submit_recaptcha_v3(post_data: web::Form<RecaptchaV3Form>) -> Result<HttpResponse> {
    // Check env
    dotenv().ok();

    // Load API key
    let secret_key = match std::env::var("RECAPTCHA_V3_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("RECAPTCHA_V3_SECRET_KEY is not set in environment."))
        }
    };

    // Send post request to api endpoint
    let client = Client::new();
    let response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[
            ("secret", secret_key),
            ("response", post_data.recaptcha_token.clone()),
        ])
        .send()
        .await;

    // Handle network or request sending errors
    let response = match response {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Error sending verification request.")
            );
        }
    };

    // Parse response JSON
    let recaptcha_response: Result<serde_json::Value, _> = response.json().await;

    match recaptcha_response {
        Ok(data) => {
            if data.get("success").and_then(serde_json::Value::as_bool) == Some(true) {
                Ok(HttpResponse::Ok().body(format!("Form submission successful: {:?}", data)))
            } else {
                Ok(HttpResponse::BadRequest()
                    .body(format!("Captcha verification failed: {:?}", data)))
            }
        }
        Err(e) => {
            eprintln!("JSON parsing error: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Error processing verification response."))
        }
    }
}

pub async fn recaptcha_enterprise(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    match std::env::var("RECAPTCHA_ENTERPRISE_SITE_KEY") {
        Ok(sitekey) => {
            context.insert("site_key", &sitekey);

            let rendered = tera
                .render("captchas/recaptcha_enterprise.html", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Err(e) => {
            eprintln!("Environment variable error: {:?}", e);
            Ok(HttpResponse::InternalServerError()
                .body("Internal server error: RECAPTCHA_ENTERPRISE_SITE_KEY not found"))
        }
    }
}

pub async fn submit_recaptcha_enterprise(
    post_data: web::Form<RecaptchaEnterpriseForm>,
) -> Result<HttpResponse> {
    // Check env
    dotenv().ok();

    // Load API key
    let secret_key = match std::env::var("RECAPTCHA_ENTERPRISE_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("RECAPTCHA_ENTERPRISE_SECRET_KEY is not set in environment."))
        }
    };

    // Load sitekey
    let site_key = match std::env::var("RECAPTCHA_ENTERPRISE_SITE_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("RECAPTCHA_ENTERPRISE_SITE_KEY is not set in environment."))
        }
    };

    let endpoint = format!("https://recaptchaenterprise.googleapis.com/v1/projects/test-project-1706648291888/assessments?key={}", secret_key);

    // Send post request to api endpoint
    let client = Client::new();
    let response = client
        .post(&endpoint)
        .json(&serde_json::json!({
            "event": {
                "token": post_data.recaptcha_token.clone(),
                "siteKey": site_key,
            }
        }))
        .send()
        .await;

    // Handle network or request sending errors
    let response = match response {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Error sending verification request.")
            );
        }
    };

    // Parse response JSON
    let recaptcha_response: Result<serde_json::Value, _> = response.json().await;

    match recaptcha_response {
        Ok(data) => {
            if data
                .get("tokenProperties")
                .and_then(|tp| tp.get("valid"))
                .and_then(Value::as_bool)
                == Some(true)
            {
                Ok(HttpResponse::Ok().body(format!("Form submission successful: {:?}", data)))
            } else {
                Ok(HttpResponse::BadRequest().body(format!("Form submission failed: {:?}", data)))
            }
        }
        Err(e) => {
            eprintln!("JSON parsing error: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Error processing verification response."))
        }
    }
}

pub async fn captcha_fox(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    match std::env::var("CAPTCHA_FOX_SITE_KEY") {
        Ok(sitekey) => {
            context.insert("site_key", &sitekey);

            let rendered = tera
                .render("captchas/captcha_fox.html", &context)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Err(e) => {
            eprintln!("Environment variable error: {:?}", e);
            Ok(HttpResponse::InternalServerError()
                .body("Internal server error: CAPTCHA_FOX_SITE_KEY not found"))
        }
    }
}

pub async fn submit_captcha_fox(post_data: web::Form<CaptchaFoxForm>) -> Result<HttpResponse> {
    // Check env
    dotenv().ok();

    // Load API key
    let secret_key = match std::env::var("CAPTCHA_FOX_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError()
                .body("CAPTCHA_FOX_SECRET_KEY is not set in environment."))
        }
    };

    // Send post request to api endpoint
    let client = Client::new();

    // Create the form data
    let params = [("secret", secret_key), ("response", post_data.cf_captcha_response.clone())];
    
    // Encode the parameters
    let body = serde_urlencoded::to_string(&params).expect("Failed to encode form data");

    // Perform the POST request
    let response = client
        .post("https://api.captchafox.com/siteverify")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await;

    // Handle network or request sending errors
    let response = match response {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {:?}", e);
            return Ok(
                HttpResponse::InternalServerError().body("Error sending verification request.")
            );
        }
    };

    // Check response code
    if response.status().is_success() {
        let captchafox_response: Result<serde_json::Value, _> = response.json().await;

        match captchafox_response {
            Ok(data) => {
                if data.get("success").and_then(serde_json::Value::as_bool) == Some(true) {
                    Ok(HttpResponse::Ok().body(format!("Form submission successful: {:?}", data)))
                } else {
                    Ok(HttpResponse::BadRequest()
                        .body(format!("Captcha verification failed: {:?}", data)))
                }
            }
            Err(e) => {
                eprintln!("JSON parsing error: {:?}", e);
                Ok(HttpResponse::InternalServerError()
                    .body("Error processing verification response."))
            }
        }
    } else {
        let captchafox_response: Result<serde_json::Value, _> = response.json().await;

        match captchafox_response {
            Ok(data) => {
                Ok(HttpResponse::BadRequest()
                    .body(format!("Captcha verification failed: {:?}", data)))
            }
            Err(e) => {
                eprintln!("JSON parsing error: {:?}", e);
                Ok(HttpResponse::InternalServerError()
                    .body("Error processing verification response."))
            }
        }
    }    
}

pub async fn m_captcha() -> Result<NamedFile> {
    let file = NamedFile::open("./static/html/m_captcha.html")?;
    Ok(file)
}

pub async fn playground(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let context = Context::new();

    let rendered = tera
        .render("captchas/playground.html", &context)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
