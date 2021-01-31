use yew::prelude::*;

use crate::utils::router::{AppRoute, Link};

pub struct Navigation {}

impl Component for Navigation {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Navigation {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="nav">
                <h1 class="h1">{"GME Stock Report"}</h1>
                <ul class="ul">
                    <li>
                        <Link route=AppRoute::Index>{"Home"}</Link>
                        <Link route=AppRoute::About>{"About"}</Link>
                        <Link route=AppRoute::ShouldISell>{"Should I Sell?"}</Link>
                    </li>
                </ul>
            </nav>
        }
    }
}
