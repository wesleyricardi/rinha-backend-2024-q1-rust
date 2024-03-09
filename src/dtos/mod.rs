use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
pub mod statement;
pub mod transaction;

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct UserSignInRequest {
    #[schema(example = "john.doe")]
    pub username: Option<String>,
    #[schema(example = "johndoe@company.com")]
    pub email: Option<String>,
    #[schema(example = "+1 151 999-9999")]
    pub telephone: Option<String>,
    #[schema(example = "12345678")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct UserRegistrationRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe")]
    pub username: String,
    #[schema(example = "1999-12-31")]
    pub birth_date: String,
    #[schema(example = 1)]
    pub gender_id: i32,
    #[schema(example = "12345678")]
    pub password: String,
    #[schema(example = "12345678")]
    pub password_repetition: String,
    #[schema(example = "153 W 57th St")]
    pub address_street: String,
    #[schema(example = "manhattan")]
    pub address_neighborhood: String,
    #[schema(example = 4)]
    pub address_city_id: i32,
    #[schema(example = 10019)]
    pub address_postal_code: i32,
    #[schema(example = "johndoe@company.com")]
    pub email: String,
    #[schema(example = "+1 151 999-9999")]
    pub telephone: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ChangeUserPasswordRequest {
    #[schema(example = "77285939-bdaa-4c5f-9805-3873fca3396e")]
    pub profile_id: String,
    #[schema(example = "12345678")]
    pub old_password: String,
    #[schema(example = "87654321")]
    pub password: String,
    #[schema(example = "87654321")]
    pub password_repetition: String,
}

#[derive(Serialize, ToSchema, Clone, Debug, Deserialize)]
pub struct UserAuthenticationResponseHttp {
    #[schema(example = 1)]
    pub id: String,
    #[schema(example = "john.doe")]
    pub username: String,
    #[schema(example = "johndoe@company.com")]
    pub name: String,
    #[schema(
        example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
    )]
    pub token: String,
}
