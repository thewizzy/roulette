use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::routes::{Roulette, RouletteAdmin, RouletteDetail, User};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/roulette.css"/>

        // sets the document title
        <Title text="Welcome to Coffee Roulette"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/roulette" view=Roulette/>
                    <Route path="/roulette/:uuid" view=RouletteDetail/>
                    <Route path="/roulette/:uuid/admin/:admin_code" view=RouletteAdmin/>
                    <Route path="/user/:uuid" view=User/>

                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = create_rw_signal(0);
    let on_click = move |_| count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Coffee Roulette!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
