use std::collections::HashMap;

use yew::{function_component, html, use_state, Callback, Html, Properties};

use crate::components::{Button, TextField};

#[derive(Clone, PartialEq)]
pub struct InitiativeInfo {
    pub name: String,
    pub initiative: i64,
}

#[derive(Properties, Clone, PartialEq)]
pub struct InitiativeTrackerProps {
    pub characters_initiative: HashMap<String, InitiativeInfo>,
}

#[function_component(InitiativeTracker)]
pub fn initiative_tracker(props: &InitiativeTrackerProps) -> Html {
    let initiative_list = use_state(|| props.characters_initiative.clone());

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
        let cloned_initiative_list = initiative_list.clone();
        let new_initiative = new_initiative.clone();
        Callback::from(move |_| {
            let it = (*new_initiative).clone();
            let mut new_one = (*cloned_initiative_list).clone();
            new_one.insert(it.name.clone(), it);
            cloned_initiative_list.set(new_one);
        })
    };

    let mut characters: Vec<InitiativeInfo> = (*initiative_list).values().cloned().collect();
    characters.sort_by(|a, b| b.initiative.cmp(&a.initiative));

    html! {
        <>
            <h2>{"Initiative"}</h2>
            <div class="initiative">
                {
                    characters.iter().map(|character| {
                        let character = character.clone();
                        let name = character.name.clone();
                        let initiative = character.initiative;
                        let on_change = {
                            let cloned_initiative_list = initiative_list.clone();
                            Callback::from(move |initiative: String| {
                                let character = character.clone();
                                let initiative = initiative.parse::<i64>();
                                if let Ok(initiative) = initiative {
                                    let mut new_one = (*cloned_initiative_list).clone();
                                    new_one.insert(character.name.clone(), InitiativeInfo {
                                        name: character.name.clone(),
                                        initiative
                                    });
                                    cloned_initiative_list.set(new_one);
                                }})
                        };

                        html! {
                            <div class="initiative-row">
                                <span class="initiative-row-name">{name}</span>
                                <TextField
                                    label="Initiative"
                                    value={initiative.to_string()}
                                    on_change={on_change}
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
