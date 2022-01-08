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
    let new_initiative = use_state(|| InitiativeInfo {
        name: "".to_string(),
        initiative: 0,
    });

    let on_name = {
        let new_initiative = new_initiative.clone();
        Callback::from(move |name: String| {
            new_initiative.set(InitiativeInfo {
                name,
                ..*new_initiative
            });
        })
    };

    let on_initiative = {
        let new_initiative = new_initiative.clone();
        Callback::from(move |initiative: String| {
            let initiative = initiative.parse::<i64>();
            if let Ok(initiative) = initiative {
                new_initiative.set(InitiativeInfo {
                    name: (*new_initiative.name).to_string(),
                    initiative,
                });
            }
        })
    };

    let on_click = {
        let new_initiative = new_initiative.clone();
        Callback::from(move |_: bool| {
            gloo_console::log!("{}", new_initiative.initiative);
        })
    };

    let counter = use_state(|| 0);

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
                    <TextField label="Name" value={(*new_initiative.name).to_string()} on_change={on_name} />
                    <TextField label="Initiative" value={new_initiative.initiative.to_string()} on_change={on_initiative} />
                    <Button label="Add" icon_name={"plus".to_string()} {on_click} />
                </div>
            </div>
        </>
    }
}
