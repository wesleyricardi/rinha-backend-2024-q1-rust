use crate::{controllers, dtos};
use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("not have components defined in macro utoipa");
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Set the JWT Token"))
                    .build(),
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
  info(
    title = "Rinha Backend 2",
    contact(
      name = "Wesley Ricardi",
      email = "wesley.ricardi@outlook.com"
    )
  ),
  paths(
    controllers::statement::get_statement,
    controllers::transaction::create_transaction,
  ),
  components(
    schemas(
      dtos::statement::ResponseStatement,
      dtos::statement::Transaction,
      dtos::statement::Balance,
      dtos::transaction::RequestTransaction,
      dtos::transaction::ResponseBalance,
    )
  ),
  tags(
    (name = "Rinha Backend 2", description = "Projeto de participação da rinha backend 2")
  ),
  modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
