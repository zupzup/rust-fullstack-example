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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub owner_id: i32,
}

pub struct CreateForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_pet_name: String,
}
// TODO: state_pet_name handling for all fields

impl CreateForm {
    fn render_form(&self, owner_id: i32) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));
        html! {
            <div class=classes!("pet-form")>
                <input type="text" value={self.state_pet_name.clone()} oninput={edit_name} />
                <button onclick=self.link.callback(move |_| Msg::MakeReq(owner_id))>{"Submit"}</button>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<PetResponse, anyhow::Error>),
    EditName(String),
}

impl Component for CreateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state_pet_name: String::new(),
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form(self.props.owner_id) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let body = PetRequest {
                    name: self.state_pet_name.clone(),
                    animal_type: "cat".to_string(),
                    color: Some("lucky".to_string()),
                };
                let req = Request::post(&format!("http://localhost:8000/owner/{}/pet", id))
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<PetResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("pet created: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: format!("/owner/{}/pet", self.props.owner_id),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                ConsoleService::info(&format!("input: {:?}", input));
                self.state_pet_name = input;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
