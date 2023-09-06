use leptos::*;

use crate::use_app;

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

                {head.map(|head| head())}
            </head>

            <body>{children()}</body>
        </html>
    }
}
