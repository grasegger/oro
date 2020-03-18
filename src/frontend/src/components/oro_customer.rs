use crate::components::nes_field::NesField;
use crate::mite::customer::MiteCustomer;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::{html, Properties, Component, ComponentLink, Html, ShouldRender};

pub struct Customer {
#[allow(dead_code)]
    link: ComponentLink<Self>,
    _fetch_task: Option<FetchTask>,
    customer: Option<MiteCustomer>,
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub apikey: String,
    pub instance: String, 
    pub customer_id: String,
}

pub enum Msg {
    CustomerLoaded(MiteCustomer),
}

impl Customer {
    fn load_data(link: ComponentLink<Self>, customer_id: String, instance: String, apikey: String) -> Option<FetchTask> {
        let request =
            MiteCustomer::get_customer(customer_id, instance, apikey);
        Some(
            FetchService::new()
                .fetch(
                    request,
                    (move |response: Response<Json<anyhow::Result<MiteCustomer>>>| {
                        match response.into_body().0 {
                            Ok(data) => link.send_message(Msg::CustomerLoaded(data)),
                            Err(error) => console::error_1(&JsValue::from_str(&error.to_string())),
                        }
                    })
                    .into(),
                )
                .unwrap(),
        )
    }
}

impl Component for Customer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        Customer {
            link,
            props: props.clone(),
            _fetch_task: Customer::load_data(link_clone, props.customer_id, props.instance, props.apikey),
            customer: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CustomerLoaded(data) => {
                self.customer = Some(data);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
            {
                if self.customer.is_some() {
                    html! {
                        <NesField>
                            <p class="nes-text is-primary"> {self.customer.clone().unwrap().details.name } </p>
                            </NesField>
                    }
                } else {
                    html! {}
                }
            }
            </>
        }
    }
}
