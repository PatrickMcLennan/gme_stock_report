use yew::prelude::*;

pub struct FourOhFour {}

impl Component for FourOhFour {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        FourOhFour {}
    }

    fn update(&mut self, _props: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main fourohfour">
                <h1 class="h1">{"This is the 404"}</h1>
            </main>
        }
    }
} 