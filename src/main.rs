use yew::{Component, ComponentLink, Html, html, Callback};

use components::play_board;


mod agents;
mod components;
mod objects;
mod settings;

pub enum MsgGame {
    RestartingWithNewSettings((
                                  settings::PlayerSetting,
                                  settings::PlayerSetting,
                                  settings::FieldSettings
                              ))
}

pub struct GameSeaBattle {
    link: ComponentLink<Self>,
    first_player_settings: settings::PlayerSetting,
    second_player_settings: settings::PlayerSetting,
    field_settings: settings::FieldSettings,

}

impl Component for GameSeaBattle {
    type Message = MsgGame;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            first_player_settings: settings::PlayerSetting {
                name: "Player".to_string(),
                is_manual_control: true,
            },
            second_player_settings: settings::PlayerSetting {
                name: "Computer".to_string(),
                is_manual_control: false,
            },
            field_settings: settings::FieldSettings {
                width_field: 10,
                height_field: 10,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            MsgGame::RestartingWithNewSettings((
                                                   first_player_settings,
                                                   second_player_settings,
                                                   field_setting
                                               )) => {
                self.first_player_settings = first_player_settings;
                self.second_player_settings = second_player_settings;
                self.field_settings = field_setting;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let parent_call: Callback<(settings::PlayerSetting,
                                   settings::PlayerSetting,
                                   settings::FieldSettings)> = self.link.callback(
            MsgGame::RestartingWithNewSettings
        );


        html! {
            <>
            <div class="form_container">
                <settings::SettingsForm
                    first_player_settings=self.first_player_settings.clone()
                    second_player_settings=self.second_player_settings.clone()
                    field_setting=self.field_settings.clone()
                    parent_call=parent_call.clone()
                />
            </div>

            <play_board::PlayBoard
                first_player_settings=self.first_player_settings.clone()
                second_player_settings=self.second_player_settings.clone()
                field_setting=self.field_settings.clone()
                parent_call=parent_call.clone()
            />
            </>
        }
    }
}


fn main() {
    yew::start_app::<GameSeaBattle>();
}