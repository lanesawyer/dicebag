use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct DeathSavingThrowsProps {
    pub saves: i64,
    pub failures: i64,
}

#[function_component(DeathSavingThrows)]
pub fn view(props: &DeathSavingThrowsProps) -> Html {
    html! {
        <section id="death-saving-throws" class="text-block">
            <h3>{ "Death Saving Throws" }</h3>
            <h4>{ "Successes" }</h4>
            <input type="checkbox" checked={props.saves >= 1} />
            <input type="checkbox" checked={props.saves >= 2} />
            <input type="checkbox" checked={props.saves >= 3}/>

            <h4>{ "Failures" }</h4>
            <input type="checkbox" checked={props.failures >= 1} />
            <input type="checkbox" checked={props.failures >= 2} />
            <input type="checkbox" checked={props.failures >= 3} />
        </section>
    }
}
