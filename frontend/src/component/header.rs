
use crate::routes::AppRoute;
use kpetrov::{
    protocol::{model::UserInfo}};

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub current_user: Option<UserInfo>,
}
