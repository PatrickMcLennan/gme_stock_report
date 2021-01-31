use yew::prelude::*;

pub struct ShouldISell {}

impl Component for ShouldISell {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ShouldISell {}
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main should-i-sell">
                <h1 class="h1">{"Should you sell?"}</h1>
                <h2 class="h2">{"No"}</h2>
            </main>
        }
    }
} 