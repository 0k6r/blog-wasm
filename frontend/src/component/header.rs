use crate::route::AppRoute;
use kpetrov::protocol::model::UserInfo;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
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

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-brand">
                        { "KPetrov" }
                    </RouterAnchor<AppRoute>>
                    <ul class="nav navbar-nav pull-xs-right">
                        <li class="nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "Home" }
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li class="nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "About" }
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li class="nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "Blog" }
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li class="nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "Contact" }
                            </RouterAnchor<AppRoute>>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
