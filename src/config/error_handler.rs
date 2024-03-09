use actix_web::{web::JsonConfig, HttpResponse, error};

pub fn error_handler() -> JsonConfig {
  JsonConfig::default().error_handler(|err,_req| {
    error::InternalError::from_response(
        err, HttpResponse::UnprocessableEntity().finish()).into()
  })
}