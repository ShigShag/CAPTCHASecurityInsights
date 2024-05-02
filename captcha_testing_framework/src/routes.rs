use actix_web::web;

use crate::views;
use crate::default_sites;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(views::landing_page))
    
    .route("/turnstile", web::get().to(views::turnstile))
    .route("/submit_turnstile", web::post().to(views::submit_turnstile))
    
    .route("/turnstile_testing", web::get().to(views::turnstile_testing))
    
    .route("/friendly_captcha", web::get().to(views::friendly_captcha))
    .route("/submit_friendly_captcha", web::post().to(views::submit_friendly_captcha))
    
    .route("/recaptcha_v2", web::get().to(views::recaptcha_v2))
    .route("/submit_recaptcha_v2", web::post().to(views::submit_recaptcha_v2))
    
    .route("/recaptcha_v3", web::get().to(views::recaptcha_v3))
    .route("/submit_recaptcha_v3", web::post().to(views::submit_recaptcha_v3))
    
    .route("/recaptcha_enterprise", web::get().to(views::recaptcha_enterprise))
    .route("/submit_recaptcha_enterprise", web::post().to(views::submit_recaptcha_enterprise))
    
    .route("/recaptcha_enterprise",web::get().to(views::recaptcha_v3))
    .route("/submit_recaptcha_enterprise", web::post().to(views::submit_recaptcha_v3))
    
    .route("/m_captcha", web::get().to(views::m_captcha))
    
    .route("/captcha_fox", web::get().to(views::captcha_fox))
    .route("/submit_captcha_fox", web::post().to(views::submit_captcha_fox))

    .route("/log_404", web::post().to(default_sites::log_404_execution))

    .route("/playground", web::get().to(views::playground));

}
