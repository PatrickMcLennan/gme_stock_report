use yew::prelude::*;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main about">
                <h1 class="h1">{"This is about page"}</h1>
            </main>
        }
    }
} 