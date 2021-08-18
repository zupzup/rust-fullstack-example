use super::super::{Anchor, AppRoute};
use common::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub owners: Option<Vec<OwnerResponse>>,
}

pub struct List {
    props: Props,
}

impl List {
    fn render_list(&self, owners: &Option<Vec<OwnerResponse>>) -> Html {
        if let Some(t) = owners {
            html! {
                <div class=classes!("list")>
                    { t.iter().map(|name| self.view_owner(name)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div class=classes!("loading")>{"loading..."}</div>
            }
        }
    }

    fn view_owner(&self, owner: &OwnerResponse) -> Html {
        html! {
            <div class=classes!("list-item")>
                <Anchor route=AppRoute::Detail(owner.id as i32)>
                    { &owner.name }
                </Anchor>
            </div>
        }
    }
}

pub enum Msg {}

impl Component for List {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_list(&self.props.owners) }
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
