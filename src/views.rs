use crate::models;

use serde::ser::Serialize;

#[derive(Serialize)]
pub struct Site<T: Serialize> {
    pub page: String,
    pub page_context: T,
    pub authenticity_token: String,
    pub current_user: Option<models::User>,
}

#[derive(Serialize)]
pub struct Error {
    pub error_code: u16,
}

pub mod home {
    #[derive(Serialize)]
    pub struct Index {
        pub i18n_fedihub: String,
        pub i18n_federated_services_without_censorship: String,
    }
}

pub mod sessions {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}

pub mod team {
    use crate::models;

    #[derive(Serialize)]
    pub struct Index {
        pub employees: Vec<models::Employee>,
    }
}

pub mod users {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}
