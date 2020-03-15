use components::nes_button::{ButtonState, NesButton};
use components::nes_container::NesContainer;
use components::nes_field::NesField;
use components::nes_form::NesForm;
use components::nes_input::{InputType, NesInput};
use web_sys::{console, HtmlInputElement};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub struct Login {
    link: ComponentLink<Self>,
    instance_ref: NodeRef,
    apikey_ref: NodeRef,
}

pub enum Msg {
    Login,
    Delete,
}

impl Login {
    fn check_credentials(&self) -> bool {
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

        // TODO check with the mite api

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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                if self.check_credentials() {
                    self.store_credentials();
                    // TODO routing
                }
            }
            Msg::Delete => {
                console::log_1(&"delete".into());
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
