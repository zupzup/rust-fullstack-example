use common::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
};

pub struct CreateForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_name: String,
}

impl CreateForm {
    fn render_form(&self) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));

        html! {
            <div class=classes!("pet-form")>
                <div>
                    <input type="text" value={self.state_name.clone()} oninput={edit_name} />
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::MakeReq)>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq,
    Resp(Result<OwnerResponse, anyhow::Error>),
    EditName(String),
}

impl Component for CreateForm {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state_name: String::new(),
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                let body = OwnerRequest {
                    name: self.state_name.clone(),
                };
                let req = Request::post("http://localhost:8000/owner")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<OwnerResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("owner created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: "/".to_string(),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                self.state_name = input;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}
