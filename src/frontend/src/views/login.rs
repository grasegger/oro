use crate::components::nes_button::{ButtonState, NesButton};
use crate::components::nes_container::NesContainer;
use crate::components::nes_field::NesField;
use crate::components::nes_form::NesForm;
use crate::components::nes_input::{InputType, NesInput};
use crate::mite::account::MiteAccount;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::HtmlInputElement;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub struct Login {
    link: ComponentLink<Self>,
    instance_ref: NodeRef,
    apikey_ref: NodeRef,
    _fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Login,
    LoginValidated(MiteAccount),
    Delete,
}

impl Login {
    fn check_credentials(&mut self) -> bool {
        let instance = self.instance_ref.cast::<HtmlInputElement>().unwrap();

        if instance.value().len() < 1 {
            let cls = instance.class_name();
            instance.set_class_name(&format!("{} is-error", cls));
        } else {
            let cls = instance.class_name();
            instance.set_class_name(&cls.replace(" is-error", ""));
        }

        let apikey = self.apikey_ref.cast::<HtmlInputElement>().unwrap();

        if apikey.value().len() < 1 {
            let cls = apikey.class_name();
            apikey.set_class_name(&format!("{} is-error", cls));
        } else {
            let cls = apikey.class_name();
            apikey.set_class_name(&cls.replace(" is-error", ""));
        }

        if instance.value().len() > 0 && apikey.value().len() > 0 {
            let request = MiteAccount::get_account(instance.value(), apikey.value());

            let link_clone = self.link.clone();

            self._fetch_task =
                Some(
                    FetchService::new()
                        .fetch(
                            request,
                            (move |response: Response<Json<anyhow::Result<MiteAccount>>>| {
                                match response.into_body().0 {
                                    Ok(data) => link_clone.send_message(Msg::LoginValidated(data)),
                                    Err(error) => {
                                        console::error_1(&JsValue::from_str(&error.to_string()))
                                    }
                                }
                            })
                            .into(),
                        )
                        .unwrap(),
                );
        }
        instance.value().len() > 0 && apikey.value().len() > 0
    }

    fn store_credentials(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window
            .session_storage()
            .expect("session storage not enabled.")
            .unwrap();

        let apikey = self.apikey_ref.cast::<HtmlInputElement>().unwrap().value();
        let instance = self
            .instance_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value();
        storage
            .set_item(&"apikey", &apikey)
            .expect("Was not able to write to sessionStorage.");
        storage
            .set_item(&"instance", &instance)
            .expect("Was not able to write to sessionStorage.");
    }
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            link: link,
            instance_ref: NodeRef::default(),
            apikey_ref: NodeRef::default(),
            _fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                self.check_credentials();
            }
            Msg::LoginValidated(_body) => {
                self.store_credentials();
                let window = web_sys::window().expect("no global `window` exists");
                window.location().set_href("./").expect("Could not change location.");
            }
            Msg::Delete => {
                let window = web_sys::window().expect("no global `window` exists");
                let storage = window
                    .session_storage()
                    .expect("session storage not enabled.")
                    .unwrap();

                storage.clear().expect("Storage couldnt be cleared.");
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <NesContainer title="Create Your Character">
                <NesForm>
                <NesField>
                <NesInput ref={self.instance_ref.clone()} identifier="instance" label="What mite realm do you serve for?" itype=InputType::Text />
                </NesField>

                <NesField>
                <NesInput ref={self.apikey_ref.clone()} identifier="apikey" label="How may you be identified?" itype=InputType::Password />
                </NesField>

                <NesField>
                <NesButton description="Start Adventure" bstate=ButtonState::Primary onsignal=self.link.callback(|_| Msg::Login) />
                </NesField>

                <NesField>
                <NesButton description="Delete Save File" bstate=ButtonState::Warning onsignal=self.link.callback(|_| Msg::Delete) />
                </NesField>
                </NesForm>
                </NesContainer>
        }
    }
}
