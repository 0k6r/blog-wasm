use yew_router::{switch::Permissive, Switch, prelude::Route};

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Home,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}
