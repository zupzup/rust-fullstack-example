use super::super::Todo;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todo_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    todo: Option<Todo>,
    fetch_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(&self, todo: &Option<Todo>) -> Html {
        match todo {
            Some(t) => {
                let completed = if t.completed {
                    Some("completed")
                } else {
                    Some("not-completed")
                };
                html! {
                    <div class=classes!("detail")>
                        <h1>{&t.title}{" ("}<span class=classes!("id")>{t.id}</span>{")"}</h1>
                        <div>{"by user "}{t.user_id}</div>
                        <div class=classes!(completed)>{if t.completed { "done" } else { "not done" }}</div>
                    </div>
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<Todo, anyhow::Error>),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq(props.todo_id));
        Self {
            props,
            link,
            todo: None,
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.todo)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let req = Request::get(&format!(
                    "https://jsonplaceholder.typicode.com/todos/{}",
                    id
                ))
                .body(Nothing)
                .expect("can make req to jsonplaceholder");

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<Todo, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::Resp(data)
                        });

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todo = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
