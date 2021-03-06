#[macro_use] extern crate log;

mod utils;
mod rtc_crypto;

use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;
use yew_mdc::components::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

struct App {
    clicked: bool,
    click_count: u32,
    onclick: Callback<MouseEvent>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            click_count: 0,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                self.click_count += 1;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            format!("Clicked {} times!", self.click_count)
        } else {
            "Click me!".to_owned()
        };

        html! {
            <>
                <Button
                    text="Log In"
                    style=button::Style::Raised
                    onclick=&self.onclick
                />
                <button onclick=&self.onclick>{ button_text }</button>
            </>
        }
    }
}

#[wasm_bindgen]
pub fn start_app() {
    wasm_logger::init(wasm_logger::Config::default());

    rtc_crypto::example();

    info!("Oh hi, how's it going? thing");
    yew::start_app::<App>();
    info!("We done now I guess");
}
