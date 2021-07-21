use crate::utils::calculate_modifier_display;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(PartialEq, Clone)]
pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

#[derive(Properties, Clone, PartialEq)]
pub struct StatBlockProps {
    pub name: String,
    pub value: i64,
    pub stat: Stat,
}

pub struct StatBlock {
    props: StatBlockProps,
}

impl Component for StatBlock {
    type Message = ();
    type Properties = StatBlockProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("stat-block", self.stat_class())>
                <div class="stat-name">{ &self.props.name }</div>
                <div class="stat-value">{ self.props.value }</div>
                <div class="stat-modifier">{ calculate_modifier_display(self.props.value) }</div>
            </div>
        }
    }
}

impl StatBlock {
    fn stat_class(&self) -> String {
        match self.props.stat {
            Stat::Strength => "strength".to_string(),
            Stat::Dexterity => "dexterity".to_string(),
            Stat::Constitution => "constitution".to_string(),
            Stat::Intelligence => "intelligence".to_string(),
            Stat::Wisdom => "wisdom".to_string(),
            Stat::Charisma => "charisma".to_string(),
        }
    }
}
