use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{ "KPetrov" }</RouterAnchor<AppRoute>>
                    <span class="attribution">
                        { "© 2020. " }
                        <a href="mailto:oku6er@gmail.com"> { "Konstantin Petrov" } </a>
                        { ". Code licensed under Apache 2.0" }
                    </span>
                </div>
            </footer>
        }
    }
}
