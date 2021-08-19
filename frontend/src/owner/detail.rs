use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub owner_id: i32,
    pub owners: Option<Vec<OwnerResponse>>,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    owner: Option<OwnerResponse>,
    pets: Option<Vec<PetResponse>>,
    fetch_task: Option<FetchTask>,
}

impl Detail {
    fn render_detail(
        &self,
        owner: &Option<OwnerResponse>,
        pets: &Option<Vec<PetResponse>>,
    ) -> Html {
        match owner {
            Some(o) => {
                html! {
                    <div class=classes!("detail")>
                        <h1>{&o.name}{" ("}<span class=classes!("id")>{o.id}</span>{")"}</h1>
                        {
                            self.view_pet_list(pets)
                        }

                    <Anchor route=AppRoute::CreatePet(o.id as i32)>
                        { "Create New Pet" }
                    </Anchor>
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

    fn view_pet_list(&self, pets: &Option<Vec<PetResponse>>) -> Html {
        match pets {
            Some(p) => {
                html! {
                    p.iter().map(|pet| self.view_pet(pet)).collect::<Html>()
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>{"loading..."}</div>
                }
            }
        }
    }

    fn view_pet(&self, pet: &PetResponse) -> Html {
        html! {
            <div class=classes!("list-item")>
                <div>{ &pet.name }</div>
                <div>{ &pet.animal_type }</div>
                <div>{ &pet.color.as_ref().unwrap_or(&String::new()) }</div>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq(i32),
    Resp(Result<Vec<PetResponse>, anyhow::Error>),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq(props.owner_id));
        let owner = match props.owners {
            Some(ref o) => o.iter().find(|owner| owner.id == props.owner_id),
            None => None,
        };
        let cloned = owner.cloned();
        Self {
            props,
            link,
            owner: cloned,
            pets: None,
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        ConsoleService::info(&format!("render detail: {:?} {:?}", self.owner, self.pets));
        html! {
            <div>
                { self.render_detail(&self.owner, &self.pets)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq(id) => {
                let req = Request::get(&format!("http://localhost:8000/owner/{}/pet", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<PetResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("data comes back: {:?}", resp));
                if let Ok(data) = resp {
                    self.pets = Some(data);
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
