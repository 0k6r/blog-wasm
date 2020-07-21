use crate::components::{footer::Footer, header::Header};
use yew_router::{prelude::*, route::Route, switch::Permissive};
use yew::{agent::Bridged, html, Bridge, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Route(Route),
}

use crate::routes::{
    fix_fragment_routes,
    AppRoute,
};

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        fix_fragment_routes(&mut route);
        App {
            current_route: AppRoute::switch(route),
            router_agent,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {
                self.current_route = AppRoute::switch(route)
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="wrapper">
                    <Header />
                        <div class="content-wrapper">
                        {
                            if let Some(route) = &self.current_route {
                                match route {
                                    AppRoute::Home => html!{"This page is WIP"},
                                    AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                    AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                                }
                            } else {
                                // 404 when route matches no component
                                html! { "No child component available" }
                            }
                        }
                        </div>
                </div>
                <Footer />
            </>
        }
    }
}
