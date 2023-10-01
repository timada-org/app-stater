use leptos::*;

#[component]
pub fn HtmxSseScript() -> impl IntoView {
    view! {
        <script
            src="https://unpkg.com/htmx.org@1.9.6/dist/ext/sse.js"
            crossorigin="anonymous"
        ></script>
    }
}
