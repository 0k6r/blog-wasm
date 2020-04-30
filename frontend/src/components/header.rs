use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <h1 class="logo">
                    <RouterAnchor<AppRoute> route=AppRoute::Home>
                        { "KONSTANTIN PETROV" }
                    </RouterAnchor<AppRoute>>
                </h1>
                <input class="switcher" type="checkbox" id="menu" />
                <label class="open" for="menu">
                    <i class="fa fa-bars"></i>
                </label>
                <nav>
                    <div class="nav-menu">
                        <ul>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                    { "HOME" }
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                    { "ABOUT" }
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                    { "BLOG" }
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                    { "CONTACT" }
                                </RouterAnchor<AppRoute>>
                            </li>
                        </ul>
                    </div>
                    <label class="close" for="menu">
                        <i class="fa fa-times"></i>
                    </label>
                </nav>
            </header>
        }
    }
}
