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

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="ft-menu">
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
                <div class="ft-social">
                    <ul>
                        <li>
                            <a href="https://twitter.com/Oku6er" aria-label="Twitter">
                                <i class="fa fa-twitter fa-fw" title="Twitter"></i>
                            </a>
                        </li>
                        <li>
                            <a href="https://github.com/0k6r" aria-label="GitHub">
                                <i class="fa fa-github-square fa-fw" title="GitHub"></i>
                            </a>
                        </li>
                        <li>
                            <a href="https://www.linkedin.com/in/konstantin-petrov/" aria-label="LinkedIn">
                                <i class="fa fa-linkedin fa-fw" title="LinkedIn"></i>
                            </a>
                        </li>
                    </ul>
                </div>

                <div class="ft-legal">
                    <ul>
                        <li>
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "Terms & Conditions" }
                            </RouterAnchor<AppRoute>>
                        </li>
                        <li>
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                { "Privacy Policy" }
                            </RouterAnchor<AppRoute>>
                        </li>
                    </ul>
                    <p>
                        { "© 2020 Konstantin Petrov. All rights reserved. \
                            Code licensed under Apache 2.0" }
                    </p>
                </div>
            </footer>
        }
    }
}
