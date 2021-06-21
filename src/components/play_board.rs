use yew::{ComponentLink, Component, Html, html, Callback, Dispatched};
use crate::components::field::PlayField;
use yew::agent::Dispatcher;
use crate::agents::agent_w;
use crate::settings;
use crate::objects::player;
use yew::services::DialogService;


pub enum Msg {
    SwitchedCellWithIndex((usize, usize)),
    EndGame(String),
}

pub struct PlayBoard {
    link: ComponentLink<Self>,
    props: settings::SettingsProps,
    pub first_player: player::Player,
    pub second_player: player::Player,
    // If "True", then the first player turn, otherwise - the second
    pub whose_move: bool,
    pub event_work: Dispatcher<agent_w::Worker>,

}

impl PlayBoard {
    /// Returns "True" if there is a winner and "lies" if there is no winner.
    fn check_winner(&self) -> bool {
        if self.first_player.get_num_living_ships() == 0 {
            self.link
                .callback(Msg::EndGame)
                .emit(self.second_player.get_name().to_string());
            return true;
        } else if self.second_player.get_num_living_ships() == 0 {
            self.link
                .callback(Msg::EndGame)
                .emit(self.first_player.get_name().to_string());
            return true;
        } else { return false; }
    }
}

impl Component for PlayBoard {
    type Message = Msg;
    type Properties = settings::SettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let first_player: player::Player = player::Player::create(
            props.first_player_settings.name.as_str(),
            props.first_player_settings.is_manual_control,
        );
        let second_player: player::Player = player::Player::create(
            props.second_player_settings.name.as_str(),
            props.second_player_settings.is_manual_control,
        );

        let event_work = agent_w::Worker::dispatcher();

        Self {
            link,
            props,
            first_player,
            second_player,
            whose_move: true,
            event_work,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            // Processes the message from the field when it is clicked.
            Msg::SwitchedCellWithIndex((idx_cell, field_number)) => {
                // We ignore messages by fields in the wrong turn.
                if (self.whose_move && field_number == 1) || (!self.whose_move && field_number == 2) {
                    return false;
                }
                match self.check_winner() {
                    true => { return false; }
                    false => {}
                }
                // If it is the turn of the 1st player,
                // then we look at the location of the ships of the 2nd player.
                let player: &mut player::Player = match self.whose_move {
                    // true - First player turn
                    true => { &mut self.second_player }
                    // false - Second player turn
                    false => { &mut self.first_player }
                };
                // Perform a "shot" on the index.
                let (number_ship, is_successful_shot, is_alive_ship) = player.process_a_shot(idx_cell);
                // If the shot turned out to be fatal, then mark all adjacent cells as "Miss".
                if !is_alive_ship {
                    if number_ship.is_some() {
                        let ship = player.get_ship_by_idx_as_mut_ref(number_ship.unwrap());
                        if ship.is_some() {
                            let area_near_ship = ship.unwrap().get_area_near_ship(None);
                            for idx in area_near_ship {
                                self.event_work.send(agent_w::Request::Shot(idx, field_number))
                            }
                        }
                    }
                }
                self.event_work.send(agent_w::Request::Shot(idx_cell, field_number));


                if !is_successful_shot {
                    // Perform a player change
                    self.whose_move = !self.whose_move;
                } else {
                    match self.check_winner() {
                        true => { return false; }
                        false => {}
                    }
                }
                // If it is the turn of the second = th player (computer),
                // then we generate the parameters of the shot.
                if !self.whose_move {
                    let new_idx_cell: usize =
                        self.first_player.generate_new_auto_shot_idx();
                    self.link
                        .callback(Msg::SwitchedCellWithIndex)
                        .emit((new_idx_cell, 1));
                }
                false
            }
            Msg::EndGame(name_winner) => {
                DialogService::alert(&*format!("{:?} winner!", name_winner));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let name_first_player = self.props.first_player_settings.name.to_string();
        let name_second_player = self.props.second_player_settings.name.to_string();
        let is_manual_control_first_player = self.props.first_player_settings.is_manual_control;
        let parent_call: Callback<(usize, usize)> = self.link.callback(Msg::SwitchedCellWithIndex);
        html! {
            <>
            <div class="wrapper">
                <div>
                    <h1 class="title">{ name_first_player + " Field" }</h1>
                    <PlayField
                        field_number=1
                        is_fog_of_war=!is_manual_control_first_player
                        is_interactive=false
                        parent_call=parent_call.clone()
                    />
                </div>
                <div>
                    <h1 class="title">{ name_second_player + " Field" }</h1>
                    <PlayField
                        field_number=2
                        is_fog_of_war=true
                        is_interactive=is_manual_control_first_player
                        parent_call=parent_call.clone()
                    ></PlayField>
                </div>
            </div>
            </>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        if first_render {
            for i in 1..3 {
                let ships_iter = if i == 1 {
                    self.first_player.get_ships_as_iter()
                } else {
                    self.second_player.get_ships_as_iter()
                };
                for ship in ships_iter {
                    for &idx_cell in ship.get_area_ship().iter() {
                        self.event_work.send(
                            agent_w::Request::GetStateCell(
                                idx_cell,
                                i,
                                "Deck".to_string(),
                            )
                        )
                    }
                }
            }
        }
    }
}