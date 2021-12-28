use yew::{function_component, html, Callback, Properties};

use crate::pages::campaign::initiative::{InitiativeInfo, InitiativeTracker};

#[derive(Properties, Clone, PartialEq)]
pub struct CampaignProps {
    pub id: i64,
}

#[function_component(CampaignPage)]
pub fn campaign_page(props: &CampaignProps) -> Html {
    let characters = vec![
        InitiativeInfo {
            name: "dude".to_string(),
            initiative: 1,
        },
        InitiativeInfo {
            name: "boi".to_string(),
            initiative: 2,
        },
    ];

    html! {
        <>
            <h1>
                { format!("Campaign number {}!", props.id) }
            </h1>
            <InitiativeTracker {characters} add={Callback::from(|_| ())}/>
        </>
    }
}
