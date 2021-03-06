use yew::prelude::*;

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Index {}
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main index">
                <h1 class="h1">{"This is The Homepage"}</h1>
            </main>
        }
    }
} 