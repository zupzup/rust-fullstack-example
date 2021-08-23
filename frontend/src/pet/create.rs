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
    state_animal_type: String,
    state_color: Option<String>,
}

impl CreateForm {
    fn render_form(&self, owner_id: i32) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));
        let edit_animal_type = self.link.callback(move |e: ChangeData| match e {
            ChangeData::Select(elem) => Msg::EditAnimalType(elem.value()),
            _ => unreachable!("only used on select field"),
        });
        let edit_color = self
            .link
            .callback(move |e: InputData| Msg::EditColor(e.value));

        html! {
            <div class=classes!("pet-form")>
                <div>
                    <input type="text" value={self.state_pet_name.clone()} oninput={edit_name} />
                </div>
                <div>
                    <select onchange={edit_animal_type}>
                        <option value="cat" selected=true>{ "Cat" }</option>
                        <option value="dog">{ "Dog" }</option>
                    </select>
                </div>
                <div>
                    <input type="text" value={self.state_color.clone()} oninput={edit_color} />
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::MakeReq(owner_id))>{"Submit"}</button>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<PetResponse, anyhow::Error>),
    EditName(String),
    EditAnimalType(String),
    EditColor(String),
}

impl Component for CreateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state_pet_name: String::new(),
            state_animal_type: String::from("cat"),
            state_color: Some(String::from("black")),
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
                    animal_type: self.state_animal_type.clone(),
                    color: self.state_color.clone(),
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
                        route: format!("/app/{}", self.props.owner_id),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                self.state_pet_name = input;
            }
            Msg::EditAnimalType(input) => {
                ConsoleService::info(&format!("input: {:?}", input));
                self.state_animal_type = input;
            }
            Msg::EditColor(input) => {
                self.state_color = Some(input);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
