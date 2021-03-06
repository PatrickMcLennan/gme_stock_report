#![recursion_limit="1000"]

mod components;
mod views;
mod utils;

use components::navigation::Navigation;
use components::footer::Footer;

use utils::router::AppRouter;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navigation />
                <AppRouter />
                <Footer />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}