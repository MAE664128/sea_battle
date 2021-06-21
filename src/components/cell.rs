use yew::{classes, html, Component, ComponentLink, Html, Callback, Bridged, Bridge};
use yew::html::Properties;
use crate::agents::agent_w;
use crate::agents::agent_w::Response;

pub enum Msg {
    ClickCell(usize),
    UpdateCell(agent_w::Response),
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    // Blank: The cell is empty.
    Blank,
    // Miss: The cell has already been shot, but the deck of the ship was not on the cell.
    Miss,
    // Deck: The deck of the ship is located on the cell.
    Deck,
    // Fire: The cell had already been shot, and the deck of the ship was knocked out.
    Fire,
}

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub idx_cell: usize,
    pub idx_field: usize,
    pub is_fog_of_war: bool,
    pub is_interactive: bool,
    pub msg_click_cell: Callback<usize>,
    // pub is_player_field: bool,
    // pub parent_call: Callback<bool>,
}

/// Describes the playing cell of the playing field.
// #[derive(Clone, Properties)]
pub struct PlayCell {
    // state: The current state of the cell.
    pub state: State,
    link: ComponentLink<Self>,
    pub props: Props,
    _producer: Box<dyn Bridge<agent_w::Worker>>,

    // coordinates: The position of the cell in the playing field in which the first cell has
    //              coordinates (0,0) and is located in the upper left angle.
    // pub coordinates: Coordinates,
}

impl Component for PlayCell {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|response| Msg::UpdateCell(response));

        let _producer = agent_w::Worker::bridge(callback);

        Self {
            link,
            state: State::Blank,
            props,
            _producer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickCell(idx_cell) => {
                // Used to switch the cell in manual mode
                if self.props.is_interactive {
                    if !self.is_fire() || !self.is_miss() {
                        self.props.msg_click_cell.emit(idx_cell);
                        // self.toggle();
                        return true
                    }
                }
                false
            }

            Msg::UpdateCell(response) => {
                match response {
                    Response::ResultStateCell(idx_cell, idx_field, text) => {
                        if idx_cell == self.props.idx_cell &&
                            idx_field == self.props.idx_field {
                            if text == "Fire" {
                                self.set_fire();
                            } else if text == "Miss" {
                                self.set_miss();
                            } else if text == "Deck" {
                                self.set_deck();
                            } else if text == "Blank" {
                                self.set_blank();
                            }
                            return true;
                        }
                    }
                    Response::ToggleCell(idx_cell, idx_field) => {
                        if idx_cell == self.props.idx_cell &&
                            idx_field == self.props.idx_field {
                            if !self.is_fire() || !self.is_miss() {
                                self.toggle();
                                return true
                            }
                        }
                    }
                    _ => {}
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let cell_status = match self.get_state() {
            State::Blank => { "cell-blank" }
            State::Miss => { "cell-miss" }
            State::Fire => { "cell-fire" }
            State::Deck => {
                if self.props.is_fog_of_war {
                    "cell-blank"
                } else { "cell-deck" }
            }
        };
        let idx_cell: usize = self.props.idx_cell;
        let result_html: Html = if self.props.is_interactive {
            let r: Html = html! {
                <div key=self.props.idx_cell class=classes!("game-cells")
                    onclick=self.link.callback(move |_| Msg::ClickCell(idx_cell))>
                    <div class=classes!(cell_status)></div>
                </div>
            };
            r
        } else {
            let r: Html = html! {
                <div key=self.props.idx_cell class=classes!("game-cells")>
                    <div class=classes!(cell_status)></div>
                </div>
            };
            r
        };
        result_html
    }
}

impl PlayCell {
    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn set_fire(&mut self) {
        self.state = State::Fire;
    }

    pub fn set_miss(&mut self) {
        self.state = State::Miss;
    }

    pub fn set_deck(&mut self) {
        self.state = State::Deck;
    }

    pub fn set_blank(&mut self) {
        self.state = State::Blank;
    }

    pub fn is_fire(&self) -> bool {
        self.state == State::Fire
    }

    pub fn is_miss(&self) -> bool {
        self.state == State::Miss
    }

    pub fn is_blank(&self) -> bool {
        self.state == State::Blank
    }

    pub fn is_deck(&self) -> bool {
        self.state == State::Deck
    }

    /// Toggle cell state
    pub fn toggle(&mut self) {
        if self.is_blank() {
            self.set_miss();
        } else if self.is_deck() {
            self.set_fire();
        }
    }
}


