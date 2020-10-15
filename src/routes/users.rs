use crate::database;
use crate::models;
use crate::forms;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[get("/users/new")]
pub fn show() -> Template {
    Template::render("users/new", &BasicTemplateContext {
        layout: "site",
    })
}

#[post("/users", data = "<form>")]
pub fn create(
    db_conn: database::DbConn,
    form: Form<forms::UserSignUp>,
) -> Result<Redirect, UserSignUpResponse>
{
    models::NewUser::from_form(form.0)?
        .save(db_conn)?;

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum UserSignUpResponse {
    #[response(status = 422)]
    InvalidForm(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    layout: &'static str,
}

impl From<validator::ValidationErrors> for UserSignUpResponse {
    fn from(_validation_errors: validator::ValidationErrors) -> Self {
        Self::InvalidForm(Template::render("users/new", &BasicTemplateContext {
            layout: "site",
        }))
    }
}

impl From<diesel::result::Error> for UserSignUpResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}