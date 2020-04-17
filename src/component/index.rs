use yew::prelude::*;

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Index
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "This is the index page. Visit /about for some more content." }</p>
        }
    }
}
