use std::borrow::Borrow;
use yew::*;
use yew::html::Scope;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use web_sys::console;
use js_sys::{JsString, Math::{random, self}};
         
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub date: String,
    pub url: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
    #[serde(rename = "Births")]
    pub births: Vec<Birth>,
    #[serde(rename = "Deaths")]
    pub deaths: Vec<Death>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub year: String,
    pub text: String,
    pub html: String,
    #[serde(rename = "no_year_html")]
    pub no_year_html: String,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Birth {
    pub year: String,
    pub text: String,
    pub html: String,
    #[serde(rename = "no_year_html")]
    pub no_year_html: String,
    pub links: Vec<Link2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link2 {
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Death {
    pub year: String,
    pub text: String,
    pub html: String,
    #[serde(rename = "no_year_html")]
    pub no_year_html: String,
    pub links: Vec<Link3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link3 {
    pub title: String,
    pub link: String,
}
#[derive(PartialEq, Properties)]
struct DataComponentProps {
    pub event: Event
}

#[function_component(DataComponent)]
fn data_component(props: &DataComponentProps) -> Html {
    let DataComponentProps { event} = props;
    html! {
        <>
        <div class = "today">
        <h1 class = "hisHead">{"Today in History"}</h1>
        <div class = "fact"> {event.text.to_owned()} </div>
        <div> {format!("Year: {:?}", event.year.to_owned())} </div>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct BirthComponentProps {
    pub birth: Birth
}

#[function_component(BirthComponent)]
fn birth_component(props: &BirthComponentProps) -> Html {
    let BirthComponentProps { birth} = props;
    html! {
        <>
        <h1 class = "births">{"Today in Births"}</h1>
        <div class = "name"> {birth.text.to_owned()} </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct DeathComponentProps {
    pub death: Death
}

#[function_component(DeathComponent)]
fn death_component(props: &DeathComponentProps) -> Html {
    let DeathComponentProps { death} = props;
    html! {
        <>
        <div class = "deaths">
        <h1 class = "dTitle">{"Today in Deaths"}</h1>
        <div class = "death"> {death.text.to_owned()} </div>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct InputComponentProps {
    pub event: Event
}

#[function_component(InputComponent)]
fn input_component(props: &InputComponentProps) -> Html {
    let InputComponentProps { event} = props;
    html! {
        <>
        <div class = "random">
        <h1 class = "randHead">{"Random Facts"}</h1>
        <div class = "randFact"> {event.text.to_owned()} </div>
        <div> {format!("Year: {:?}", event.year.to_owned())} </div>
        </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    let fact = Box::new(use_state(|| None));
    {
       let fact = fact.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fact_endpoint = format!(
                "https://history.muffinlabs.com/date"
            );
            let fetched_fact: Root = Request::get(&fact_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        fact.set(Some(fetched_fact));
        });
    }
    match &**fact {
        Some(f) => {
            f.data.events.iter().take(3).map(|event| {
                html! {
                    <DataComponent event={event.clone()}/>
                }
            }).collect()
        },
        None => html! (
            {
                "No data yet"
            }
        )
            }
}

#[function_component(App2)]
fn showBirths() -> Html {
     let birth = Box::new(use_state(|| None));
    {
       let birth = birth.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let birth_endpoint = format!(
                "https://history.muffinlabs.com/date"
            );
            let fetched_birth: Root = Request::get(&birth_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        birth.set(Some(fetched_birth));
        });
    } 


     match &**birth {
        Some(f) => {
            f.data.births.iter().take(3).map(|birth| {
                html! {
                    <BirthComponent birth={birth.clone()}/>
                }
            }).collect()
        },
        None => html! (
            {
                "No data yet"
            }
        )
            } 
}

#[function_component(App3)]
fn showDeaths() -> Html {
     let death = Box::new(use_state(|| None));
    {
       let death = death.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let death_endpoint = format!(
                "https://history.muffinlabs.com/date"
            );
            let fetched_death: Root = Request::get(&death_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        death.set(Some(fetched_death));
        });
    } 
     match &**death {
        Some(f) => {
            f.data.deaths.iter().take(3).map(|death| {
                html! {
                    <DeathComponent death={death.clone()}/>
                }
            }).collect()
        },
        None => html! (
            {
                "No data yet"
            }
        )
            } 
}

 #[function_component(InputData)]
pub fn input_container() -> Html {
    let fact = Box::new(use_state(|| None));
    let desire = {
        let fact = fact.clone();
            Callback::from(move |e: KeyboardEvent|{
                let fact = fact.clone();    
                if e.key() == "Enter"{
                    let input: InputElement = e.target_unchecked_into();
                    let value = input.value();
                    input.set_value("");
                    let fact = fact.clone();
                    wasm_bindgen_futures::spawn_local(async move{
                        let fact_endpoint = format!(
                            "https://history.muffinlabs.com/date/{month}/{date}",
                            month = value,
                            date = value
                        );
                        let fetched_fact: Root = Request::get(&fact_endpoint)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                        fact.set(Some(fetched_fact));
                        });
                }
            })
    }; 

    match &**fact {
        Some(f) => {f.data.events.iter().take(3).map(|event| {
            html! {
                   <InputComponent event = {event.clone()}/>
                  }
            }).collect()
        },
        None => html!
                {   
                 <ul>
                    <input
                        class="new-month"
                        placeholder="Enter a month, 1-12"
                        />
                    <input
                        class="new-date"
                        placeholder="Enter a date"
                        />
                    </ul> 
                }
    }
    
} 
        
#[function_component(AppInput)]
fn appInput() -> Html {
    let fact = Box::new(use_state(|| None));
    {
        let random_month = Math::random() * (12.0-1.0) + 1.0;
        let random_date = Math::random() * (29.0-1.0) + 1.0;
        let fact = fact.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fact_endpoint = format!(
                "https://history.muffinlabs.com/date/{month}/{date}",
                month = random_month,
                date = random_date
            );
            let fetched_fact: Root = Request::get(&fact_endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        fact.set(Some(fetched_fact));
        });
    }
    match &**fact {
        Some(f) => {
            f.data.events.iter().take(3).map(|event| {
                html! {
                    <InputComponent event={event.clone()}/>
                }
            }).collect()
        },
        None => html! (
            {
                "There is no histroical events for the random data"
            }
        )
            }
}

fn main() {
    yew::start_app::<App>();
    yew::start_app::<App2>();
    yew::start_app::<App3>();
    yew::start_app::<AppInput>();
}
