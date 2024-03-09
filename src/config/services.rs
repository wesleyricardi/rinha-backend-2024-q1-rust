use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers;

use super::swagger::ApiDoc;

pub fn services_config(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::home::index)
        .service(controllers::statement::get_statement)
        .service(controllers::transaction::create_transaction)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .default_service(web::to(controllers::not_found));
}
