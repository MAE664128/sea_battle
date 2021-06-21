use yew::{ComponentLink, Component, Html, html, Callback, InputData, ChangeData};
use yew::html::Properties;

pub enum MsgSettings {
    // Show setting
    SettingShowed(bool),
    PlayerNameChange(InputData),
    _PlayerTypeControlChange(ChangeData),
    Submit,

}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SettingsProps {
    pub first_player_settings: PlayerSetting,
    pub second_player_settings: PlayerSetting,
    pub field_setting: FieldSettings,
    pub parent_call: Callback<(
        PlayerSetting,
        PlayerSetting,
        FieldSettings)>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
/// Options available to customize the player
pub struct PlayerSetting {
    pub name: String,
    // is_manual_control: If "False", then the computer controls
    pub is_manual_control: bool,
}

#[derive(Clone, Debug, PartialEq, Properties)]
/// Options available to customize the fields
pub struct FieldSettings {
    pub width_field: usize,
    pub height_field: usize,
}

pub struct SettingsForm {
    link: ComponentLink<Self>,
    props: SettingsProps,
    visible: bool,
    first_player_settings: PlayerSetting,
    second_player_settings: PlayerSetting,
    field_setting: FieldSettings,
}

impl Component for SettingsForm {
    type Message = MsgSettings;
    type Properties = SettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let first_player_settings = props.first_player_settings.clone();
        let second_player_settings = props.second_player_settings.clone();
        let field_setting = props.field_setting.clone();
        Self {
            link,
            props,
            visible: false,
            first_player_settings,
            second_player_settings,
            field_setting,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            MsgSettings::SettingShowed(is_show) => {
                self.visible = is_show;
                true
            }
            MsgSettings::PlayerNameChange(e) => {
                self.first_player_settings.name = e.value;
                true
            }
            MsgSettings::_PlayerTypeControlChange(e) => {
                match e {
                    ChangeData::Value(_) => {}
                    ChangeData::Select(el) => {
                        if el.value() == "1" {
                            self.first_player_settings.is_manual_control = true;
                        } else {
                            self.first_player_settings.is_manual_control = false;
                        }
                    }
                    ChangeData::Files(_) => {}
                };
                true
            }
            MsgSettings::Submit => {
                self.props.parent_call.emit(
                    (self.first_player_settings.clone(),
                    self.second_player_settings.clone(),
                    self.field_setting.clone()));
                self.link.callback(MsgSettings::SettingShowed).emit(false);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let is_need_to_show: bool = !self.visible;
        let text_button: String = if self.visible {
            "Hide Settings".to_string()
        } else {
            "Show Settings".to_string()
        };

        // let select_type_control = if self.first_player_settings.is_manual_control {
        //     html! {
        //         <select class="element select medium" id="element_2" name="element_2"
        //         onchange=self.link.callback(|e| MsgSettings::PlayerTypeControlChange(e))>
        //         <option value="1" selected=true >{ "Manual control" }</option>
        //         <option value="2" >{ "Computer control" }</option>
        //         </select>
        //     }
        // } else {
        //     html! {
        //         <select class="element select medium" id="element_2" name="element_2"
        //         onchange=self.link.callback(|e| MsgSettings::PlayerTypeControlChange(e))>
        //         <option value="1" >{ "Manual control" }</option>
        //         <option value="2" selected=true >{ "Computer control" }</option>
        //         </select>
        //     }
        // };

        let form_settings = if self.visible {
            html! {
                <>
                <ul>
                    <li id="li_1" >
                        <label class="description" for="element_1">{ "Name Player" }</label>
                        <div>
                            <input
                                id="element_1"
                                name="element_1"
                                class="element text medium"
                                type="text"
                                maxlength="255"
                                value=self.first_player_settings.name.to_owned()
                                oninput=self.link.callback(|e| MsgSettings::PlayerNameChange(e))
                            />
                        </div>
                    </li>
                    // <li id="li_2" >
                    //     <label class="description" for="element_2">{ "Control type" }</label>
                    //     <div>
                    //         { select_type_control }
                    //     </div>
                    // </li>
                    <li class="buttons">
                        <button onclick=self.link.callback(|_| MsgSettings::Submit)>
                            { "Submit" }
                        </button>
                    </li>
                </ul>
                </>
            }
        } else { html! {  } };

        html! {
            <>
            <div class="wrapper">
                <div>
                    <button onclick=self.link.callback(move |_|
                        MsgSettings::SettingShowed(is_need_to_show))>
                        { text_button }
                    </button>
                </div>
                <div>
                    { form_settings }
                </div>
            </div>
            </>
        }
    }
}