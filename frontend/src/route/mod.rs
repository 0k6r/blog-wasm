use yew_router::Switch;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/"]
    Home,
}
