use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct CampaignProps {
    pub id: i64,
}

#[function_component(CampaignPage)]
pub fn campaign_page(props: &CampaignProps) -> Html {
    html! {
        <h1>
            { format!("Campaign number {}!", props.id) }
        </h1>
    }
}
