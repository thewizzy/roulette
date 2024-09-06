use crate::error_template::ErrorTemplate;
use leptos::*;
use leptos_router::*;

#[component]
pub fn User() -> impl IntoView {
    use crate::models::user::get_user;
    let params = use_params_map();
    let navigate = use_navigate();

    create_effect(move |_| {
        if params.with(|p| p.get("uuid").cloned().unwrap_or_default()).is_empty() {
            navigate("/", Default::default());
        }
    });

    let user_data = create_resource(
        move || params.with(|p| p.get("uuid").cloned().unwrap_or_default()),
        move |uuid| get_user(uuid),
    );

    view!{
        <h1>User Details</h1>
        <Transition fallback=move || view! { <p>"Loading user..."</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let user = {
						move || {
							if user_data.get().is_some() {
								match user_data.get().unwrap() {
									Err(e) => {
										view! {
											<pre class="error">"Server Error: " {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(user) => {
										view! { <p>{user.first_name}, {user.email}</p> }
											.into_view()
									}
								}
							} else {
								view! { <div>Nothing found</div> }.into_view()
							}
						}
					};
					view! { <div>{user}</div> }
				}}
			</ErrorBoundary>
		</Transition>
    }
}