use crate::error_template::ErrorTemplate;
use crate::models::roulette::Roulette;

use leptos::*;
use leptos_router::*;

#[component]
pub fn RouletteDetail() -> impl IntoView {
	use crate::models::roulette::get_roulette;

    let params = use_params_map();
    let navigate = use_navigate();

    create_effect(move |_| {
        if params.with(|p| p.get("uuid").cloned().unwrap_or_default()).is_empty() {
            navigate("/roulette", Default::default());
        }
    });

    let roulette_data = create_resource(
        move || params.with(|p| p.get("uuid").cloned().unwrap_or_default()),
        move |uuid| get_roulette(uuid),
    );

    view! {
		<h1>Roulette Details</h1>
		<Transition fallback=move || view! { <p>"Loading roulette..."</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let roulette = {
						move || {
							if roulette_data.get().is_some() {
								match roulette_data.get().unwrap() {
									Err(e) => {
										view! {
											<pre class="error">"Server Error: " {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(roulette) => {
										view! { <p>{roulette.name}, {roulette.description}</p> }
											.into_view()
									}
								}
							} else {
								view! { <div>Nothing found</div> }.into_view()
							}
						}
					};
					view! { <div>{roulette}</div> }
				}}
			</ErrorBoundary>
		</Transition>
	}
}


#[component]
pub fn Roulette() -> impl IntoView {

    view!{
        <h1>Roulette Page</h1>
    }

}