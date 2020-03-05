use crate::components::{header::Header, footer::Footer};
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive};

pub struct App;
use crate::routes::AppRoute;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <div>
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute | {
                            match switch {
                                AppRoute::Home => html!{<h2>{"Home"}</h2>},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        } )
                        redirect = Router::redirect(|route: Route<()>| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
                <Footer />
            </>
        }
    }
}
