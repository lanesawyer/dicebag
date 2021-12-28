use yew::{function_component, html, use_state, Callback, Html, Properties};

use crate::components::{Button, TextField};

#[derive(Clone, PartialEq)]
pub struct InitiativeInfo {
    pub name: String,
    pub initiative: i64,
}

#[derive(Properties, Clone, PartialEq)]
pub struct InitiativeTrackerProps {
    pub characters: Vec<InitiativeInfo>,
    pub add: Callback<bool>,
}

#[function_component(InitiativeTracker)]
pub fn initiative_tracker(props: &InitiativeTrackerProps) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let mut characters = props.characters.clone();
    characters.sort_by(|a, b| b.initiative.cmp(&a.initiative));

    html! {
        <>
            <h2>{"Initiative"}</h2>
            <div class="initiative">
                {
                    characters.iter().map(|character| {
                        let onclick = {
                            let counter = counter.clone();
                            Callback::from(move |_| counter.set(*counter + 1))
                        };
                        html! {
                            <div class="initiative-row">
                                <span class="initiative-row-name">{&character.name}</span>
                                <TextField
                                    label="Initiative"
                                    value={character.initiative.to_string()}
                                    on_change={onclick}
                                />
                            </div>
                        }
                    }).collect::<Html>()
                }
                <div class="initiative-row">
                    <TextField label="Name" value={"1".to_string()} on_change={&onclick} />
                    <TextField label="Initiative" value={"1".to_string()} on_change={onclick} />
                    <Button label="Add" icon_name={"plus".to_string()} on_click={&props.add} />
                </div>
            </div>
        </>
    }
}
