#![recursion_limit = "256"]

use common::*;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::html;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod owner;
mod pet;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {
    link: ComponentLink<Self>,
    owners: Option<Vec<OwnerResponse>>,
    fetch_task: Option<FetchTask>,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<OwnerResponse>, anyhow::Error>),
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            owners: None,
            fetch_task: None,
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
        false
    }

    fn view(&self) -> Html {
        let owners = self.owners.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        ConsoleService::info(&format!("render FullStackApp: {:?}", owners));
        html! {
            <div class=classes!("app")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Detail(owner_id) => {
                                    html! {
                                        <div>
                                            <owner::detail::Detail owners=owners.clone() owner_id=owner_id/>
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <div class=classes!("refresh")>
                                                <button onclick=cb.clone()>
                                                    { "refresh" }
                                                </button>
                                            </div>
                                            <owner::list::List owners=owners.clone()/>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<FullStackApp>::new().mount_to_body();
}
