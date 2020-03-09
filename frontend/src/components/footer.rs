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
                // <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{ "KPetrov" }</RouterAnchor<AppRoute>>
		        <ul class="ft-social">
			        <li>
			            <a href="#" aria-label="Twitter">
			                <i class="fa fa-twitter fa-fw" title="Twitter"></i>
			            </a>
			        </li>
			        <li>
			            <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                            <i class="fa fa-github-square fa-fw" title="GitHub"></i>
                        </RouterAnchor<AppRoute>>
			        </li>
			        <li>
			            <a href="#" aria-label="LinkedIn">
			                <i class="fa fa-linkedin fa-fw" title="LinkedIn"></i>
			            </a>
			        </li>
	            </ul>

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
                        <li>
                            { "© 2020 Konstantin Petrov. All rights reserved. \
                            Code licensed under Apache 2.0" }
                        </li>
                    </ul>
                </div>
            </footer>
        }
    }
}
