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
                // <section class="ft-menu">
                    <div class="ft-menu">
                        <ul>
                          <li><a href="#">{ "Home" }</a></li>
                          <li><a href="#">{ "About" }</a></li>
                          <li><a href="#">{ "Blog" }</a></li>
                          <li><a href="#">{ "Contact" }</a></li>
                        </ul>
                    </div>
                    // <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{ "KPetrov" }</RouterAnchor<AppRoute>>
                // </section>

                // <section class="ft-social">
		            <ul class="ft-social">
			            <li>
			                <a href="#" aria-label="Twitter">
			                    <i class="fa fa-twitter fa-fw" title="Twitter"></i>
			                </a>
			            </li>
			            <li>
			                <a chref="#" aria-label="GitHub">
			                    <i class="fa fa-github-square fa-fw" title="GitHub"></i>
			                </a>
			            </li>
			            <li>
			                <a href="#" aria-label="LinkedIn">
			                    <i class="fa fa-linkedin fa-fw" title="LinkedIn"></i>
			                </a>
			            </li>
	                </ul>
                // </section>

                // <section class="ft-legal">
                    <ul class="ft-legal">
                        <li><a href="#">{ "Terms & Conditions" }</a></li>
                        <li><a href="#">{ "Privacy Policy" }</a></li>
                        <li>
                            { "© 2020 Konstantin Petrov. All rights reserved. \
                            Code licensed under Apache 2.0" }
                        </li>
                    </ul>
                // </section>
            </footer>
        }
    }
}
