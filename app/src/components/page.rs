use leptos::*;

use crate::context::use_app;

#[component]
pub fn Page(
    children: Children,
    #[prop(into, default = "en".to_owned())] lang: String,
    #[prop(into, default = "Timada Starter App".to_owned())] title: String,
    #[prop(optional)] head: Option<Children>,
) -> impl IntoView {
    let app = use_app();

    view! {
        <html lang=lang>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <title>{title}</title>

                <link rel="icon" href=app.create_static_url("favicon.ico")/>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/@unocss/reset/normalize.min.css"
                />

                <script src="https://unpkg.com/htmx.org@1.9.5"></script>
                <script src="https://unpkg.com/htmx.org/dist/ext/response-targets.js"></script>
                <script src="https://unpkg.com/hyperscript.org@0.9.11"></script>

                {head.map(|head| head())}
            </head>

            <body hx-ext="response-targets">{children()}</body>
        </html>
    }
}
