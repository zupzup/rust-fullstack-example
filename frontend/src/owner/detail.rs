use super::super::{Anchor, AppRoute};
use common::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub owner_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    pets: Option<Vec<PetResponse>>,
    owner: Option<OwnerResponse>,
    fetch_pets_task: Option<FetchTask>,
    fetch_owner_task: Option<FetchTask>,
    delete_pet_task: Option<FetchTask>,
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

                    <br />
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
        let id = pet.id;
        let owner_id = self.props.owner_id;
        html! {
            <div class=classes!("list-item", "pet")>
                <div><b>{ &pet.name }</b> { " (" } <button onclick=self.link.callback(move |_| Msg::MakeDeletePetReq(owner_id, id))>{"Delete"}</button> {")"}</div>
                <div>{ &pet.animal_type }</div>
                <div>{ &pet.color.as_ref().unwrap_or(&String::new()) }</div>
            </div>
        }
    }
}

pub enum Msg {
    MakePetsReq(i32),
    MakeOwnerReq(i32),
    MakeDeletePetReq(i32, i32),
    RespPets(Result<Vec<PetResponse>, anyhow::Error>),
    RespOwner(Result<OwnerResponse, anyhow::Error>),
    RespDeletePet(Response<Json<Result<(), anyhow::Error>>>, i32),
}

impl Component for Detail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakePetsReq(props.owner_id));
        link.send_message(Msg::MakeOwnerReq(props.owner_id));
        Self {
            props,
            link,
            owner: None,
            pets: None,
            fetch_pets_task: None,
            fetch_owner_task: None,
            delete_pet_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.owner, &self.pets)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakePetsReq(id) => {
                let req = Request::get(&format!("http://localhost:8000/owner/{}/pet", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<PetResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespPets(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_pets_task = Some(task);
                ()
            }
            Msg::MakeOwnerReq(id) => {
                let req = Request::get(&format!("http://localhost:8000/owner/{}", id))
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<OwnerResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RespOwner(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_owner_task = Some(task);
                ()
            }
            Msg::MakeDeletePetReq(owner_id, pet_id) => {
                let req = Request::delete(&format!(
                    "http://localhost:8000/owner/{}/pet/{}",
                    owner_id, pet_id
                ))
                .body(Nothing)
                .expect("can make req to backend");

                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::RespDeletePet(response, pet_id)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.delete_pet_task = Some(task);
                ()
            }
            Msg::RespPets(resp) => {
                if let Ok(data) = resp {
                    self.pets = Some(data);
                }
            }
            Msg::RespOwner(resp) => {
                if let Ok(data) = resp {
                    self.owner = Some(data);
                }
            }
            Msg::RespDeletePet(resp, id) => {
                if resp.status().is_success() {
                    self.pets = self
                        .pets
                        .as_ref()
                        .map(|pets| pets.into_iter().filter(|p| p.id != id).cloned().collect());
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
