use yew::prelude::*;

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
                {"this is the nav"}
            </nav>
        }
    }
}
