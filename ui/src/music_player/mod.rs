use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::Button;

enum PlayStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub enum Msg {
    Play,
    Pause,
}

pub struct MusicPlayer {
    link: ComponentLink<Self>,
    play_status: PlayStatus,
}

impl Component for MusicPlayer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            play_status: PlayStatus::Stopped,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let audio = yew::utils::document()
            .get_element_by_id("player-audio")
            .expect("audio element with id=player-audio should exist");

        let audio_html_media: HtmlMediaElement = audio.dyn_into().expect("cast should succeed");

        match msg {
            Msg::Play => match audio_html_media.play() {
                Ok(_) => self.play_status = PlayStatus::Playing,
                Err(error) => {
                    yew::services::ConsoleService::log(&format!(
                        "error playing audio: {:?}",
                        error
                    ));
                }
            },
            Msg::Pause => match audio_html_media.pause() {
                Ok(_) => self.play_status = PlayStatus::Paused,
                Err(error) => {
                    yew::services::ConsoleService::log(&format!(
                        "error pausing audio: {:?}",
                        error
                    ));
                }
            },
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // TODO: Multiple players for different scenarios
        let fight_button = match self.play_status {
            PlayStatus::Playing => {
                html! { <Button label="Fight" icon_name="media-pause".to_string() on_click=self.link.callback(|_| Msg::Pause) /> }
            }
            PlayStatus::Paused | PlayStatus::Stopped => {
                html! { <Button label="Fight" icon_name="media-play".to_string() on_click=self.link.callback(|_| Msg::Play) /> }
            }
        };

        html! {
            <section id="music-player" class="text-block">
                <h3>{ "MusicPlayer"}</h3>
                <audio id="player-audio">
                    <source src="/assets/music/BrokeForFree-NightOwl.mp3" type="audio/mpeg" />
                </audio>
                { fight_button }
            </section>
        }
    }
}
