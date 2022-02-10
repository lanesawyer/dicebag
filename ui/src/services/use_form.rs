use validator::{Validate, ValidationError};
use web_sys::FocusEvent;
use yew::{function_component, html, Callback, use_state};

#[derive(Debug, Clone, Validate)]
struct Login {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[function_component(FormTest)]
pub fn form_test() -> Html {
    let form = use_form();

    let on_submit = |values: Login| {
        gloo_console::log!(values.email);
    };

    html! {
        <form onsubmit={form.clone().handleSubmit(on_submit)}>
            <input type="text" name={form.data.email.clone()} onchange={form.change["email"]}/>
            <input type="password" name={form.data.password.clone()} />
            // {errors.exampleRequired && <span>This field is required</span>}
            <input type="submit" value="Login" />
        </form>
    }
}

#[derive(Clone)]
pub struct UseForm {
    data: Login,
    errors: Vec<bool>,
    handleSubmit: Callback<FocusEvent>,
}

impl UseForm {
    fn handleSubmit<F>(self, func: F) -> Callback<FocusEvent>
    where
        F: Fn(Login) + 'static,
    {
        match self.data.validate() {
            Ok(_) => {
                gloo_console::log!("good");
                Callback::from(move |e: FocusEvent| {
                    e.prevent_default();
                    func(self.data.clone());
                })
            },
            Err(errors) => {
                let wat: Vec<_> = errors.field_errors().iter().collect();
                gloo_console::log!("bad");
                Callback::from(move |e: FocusEvent| {
                    e.prevent_default();
                })
            },
        }
    }
}

pub fn use_form() -> UseForm {
    let new_email = use_state(|| "".to_string());

    let on_change_email = {
        let new_email = new_email.clone();
        Callback::from(move |input| new_email.set(input))
    };

    UseForm {
        data: Login {
            email: "tester@gmail.com".to_string(),
            password: "123456789".to_string(),
        },
        errors: vec![],
        handleSubmit: Callback::from(|_| ()),
    }
}
