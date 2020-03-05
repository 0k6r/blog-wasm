use yew_router::{switch::Permissive, Switch};

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}
