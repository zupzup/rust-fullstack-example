use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct List {
    fetch_task: Option<FetchTask>,
    owners: Option<Vec<OwnerResponse>>,
    link: ComponentLink<Self>,
}

impl List {
    fn render_list(&self) -> Html {
        if let Some(t) = &self.owners {
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

pub enum Msg {
    MakeReq,
    Resp(Result<Vec<OwnerResponse>, anyhow::Error>),
}

impl Component for List {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            fetch_task: None,
            link,
            owners: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_list() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.owners = None;
                let req = Request::get("http://localhost:8000/owner")
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<OwnerResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.owners = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}
