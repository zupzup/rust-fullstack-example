use super::super::{Anchor, AppRoute, Todo};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todos: Option<Vec<Todo>>,
}

pub struct List {
    props: Props,
}

impl List {
    fn render_list(&self, todos: &Option<Vec<Todo>>) -> Html {
        if let Some(t) = todos {
            html! {
                <div class=classes!("list")>
                    { t.iter().map(|todo| self.view_todo(todo)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div class=classes!("loading")>{"loading..."}</div>
            }
        }
    }

    fn view_todo(&self, todo: &Todo) -> Html {
        let completed = if todo.completed {
            Some("completed")
        } else {
            None
        };
        html! {
            <div class=classes!("list-item", completed)>
                <Anchor route=AppRoute::Detail(todo.id as i32)>
                    { &todo.title }
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
                { self.render_list(&self.props.todos)}
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
