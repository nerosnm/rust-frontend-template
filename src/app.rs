use yew::prelude::*;
use yew_router::{router::Router, Switch};

use crate::component::{About, Index};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/"]
    Index,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::About=> html!{ <About /> },
                        AppRoute::Index => html!{ <Index /> },
                    }
                })
            />
        }
    }
}
