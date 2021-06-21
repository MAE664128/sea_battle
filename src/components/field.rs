use yew::{ html, Component, ComponentLink, Html, ShouldRender, Properties, Callback};
use crate::components::cell::PlayCell;

pub struct Callbacks {
    on_click_cell: Callback<usize>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PlayFieldProps {
    pub field_number: usize,
    pub is_interactive: bool,
    pub is_fog_of_war: bool,
    pub parent_call: Callback<(usize, usize)>,
}

pub enum Msg {
    ClickField(usize),
}

pub struct PlayField {
    // link: ComponentLink<Self>,
    props: PlayFieldProps,
    // active: bool,
    callbacks: Callbacks,
}

impl Component for PlayField {
    type Message = Msg;
    type Properties = PlayFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callbacks = Callbacks {
            on_click_cell: link.callback(|cell| Msg::ClickField(cell)),
        };

        Self {
            // link,
            props,
            // active: false,
            callbacks,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickField(idx) => {
                self.props.parent_call.emit((idx, self.props.field_number));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let width_field: usize = 10;
        let height_field: usize = 10;
        let cell_rows = (0..height_field).into_iter().map(|idx_row| {
            html! {
                <div key=idx_row class="game-row">
                    {
                        for (0..width_field).into_iter().map(|idx_col| {
                        html! {
                            <PlayCell
                            idx_cell=idx_row * width_field + idx_col
                            idx_field=self.props.field_number
                            is_fog_of_war=self.props.is_fog_of_war
                            is_interactive=self.props.is_interactive
                            msg_click_cell=self.callbacks.on_click_cell.clone()
                            />
                        }
                    })
                    }
                </div>
            }
        });
        html! {
            <div class="game-field">
                { for cell_rows }
            </div>
        }
    }


}
