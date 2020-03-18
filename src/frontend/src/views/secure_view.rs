use crate::components::nes_container::NesContainer;
use crate::components::nes_link_button::ButtonState;
use crate::components::nes_link_button::NesLinkButton;
use yew::{html, Html};

pub trait SecureView {
    fn logged_in(&self) -> bool {
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window
            .session_storage()
            .expect("session storage not enabled.")
            .unwrap();
        storage.length().unwrap() > 1
    }

    fn render_secured_view(&self) -> Html {
        unimplemented!();
    }

    fn get_api_key() -> String {
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window
            .session_storage()
            .expect("session storage not enabled.")
            .unwrap();

        storage.get_item("apikey").unwrap().unwrap().to_string()
    }

    fn get_instance() -> String {
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window
            .session_storage()
            .expect("session storage not enabled.")
            .unwrap();

        storage.get_item("instance").unwrap().unwrap().to_string()
    }

    fn secured_view(&self) -> Html {
        if self.logged_in() {
            self.render_secured_view()
        } else {
            html! {
                <NesContainer dark=true title="Uh-Oh!">
                    <p>{"I don't remember you. :( Please create a new character."}</p>
                    <NesLinkButton bstate=ButtonState::Primary href="./login" description="Create Character" />
                </NesContainer>
            }
        }
    }
}
