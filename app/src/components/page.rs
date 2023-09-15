use leptos::*;

use crate::state::use_app;

#[component]
pub fn Page(
    children: Children,
    #[prop(into, default = "Timada Starter App".to_owned())] title: String,
    #[prop(optional)] head: Option<Children>,
) -> impl IntoView {
    let app = use_app();

    view! {
        <html lang=app.lang.to_owned()>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <title>{title}</title>

                <link rel="icon" href=app.create_static_url("favicon.ico")/>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/@unocss/reset/normalize.min.css"
                    crossorigin="anonymous"
                />

                <script src="https://unpkg.com/htmx.org@1.9.5" crossorigin="anonymous"></script>
                <script
                    src="https://unpkg.com/htmx.org/dist/ext/response-targets.js"
                    crossorigin="anonymous"
                ></script>
                <script
                    src="https://unpkg.com/hyperscript.org@0.9.11"
                    crossorigin="anonymous"
                ></script>

                {head.map(|head| head())}
            </head>

            <body>{children()}</body>
        </html>
    }
}
