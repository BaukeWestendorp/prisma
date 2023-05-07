use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <span>{ "Hello Prisma" }</span>
        </main>
    }
}
