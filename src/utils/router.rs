use yew::prelude::*;
use yew_router::prelude::*;

use crate::views;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/should-i-sell"]
    ShouldISell,
    #[to = "/"]
    Index
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|route: AppRoute| match route {
            AppRoute::Index => html! { <views::Index/> },
            AppRoute::About => html! { <views::About/> },
            AppRoute::ShouldISell => html! { <views::ShouldISell/> },
        });
        html! {
            <Router<AppRoute, ()> render=render_func />

        }
    }
}