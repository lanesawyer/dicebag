use yew::{Component, ComponentLink, Html, ShouldRender, html, Properties};

#[derive(Properties, Clone)]
pub struct StatBlock {
    pub name: String,
    pub value: i32,
}

impl Component for StatBlock {
    type Message = ();
    type Properties = StatBlock;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: props.name,
            value: props.value,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="stat-block">
                <div>{ &self.name }</div>
                <div>{ self.value }</div>
                <div>{ calculate_modifier(self.value) }</div>
            </div>
        }
    }
}

fn calculate_modifier(stat: i32) -> i32 {
    match stat {
        1 => -5,
        2 | 3 => -4,
        4 | 5 => -3,
        6 | 7 => -2,
        8 | 9 => -1,
        10 | 11 => 0,
        12 | 13 => 1,
        14 | 15 => 2,
        16 | 17 => 3,
        18 | 19 => 4,
        20 => 5,
        _ => panic!("Ability scores must be between 1 and 20")
    }
}

#[cfg(test)]
mod tests {
    use crate::stat_block::calculate_modifier;

    #[test]
    fn calculate_modifier_works() {
        assert_eq!(calculate_modifier(1), -5);
        assert_eq!(calculate_modifier(2), -4);
        assert_eq!(calculate_modifier(3), -4);
        assert_eq!(calculate_modifier(4), -3);
        assert_eq!(calculate_modifier(5), -3);
        assert_eq!(calculate_modifier(6), -2);
        assert_eq!(calculate_modifier(7), -2);
        assert_eq!(calculate_modifier(8), -1);
        assert_eq!(calculate_modifier(9), -1);
        assert_eq!(calculate_modifier(10), 0);
        assert_eq!(calculate_modifier(11), 0);
        assert_eq!(calculate_modifier(12), 1);
        assert_eq!(calculate_modifier(13), 1);
        assert_eq!(calculate_modifier(14), 2);
        assert_eq!(calculate_modifier(15), 2);
        assert_eq!(calculate_modifier(16), 3);
        assert_eq!(calculate_modifier(17), 3);
        assert_eq!(calculate_modifier(18), 4);
        assert_eq!(calculate_modifier(19), 4);
        assert_eq!(calculate_modifier(20), 5);
    }
}