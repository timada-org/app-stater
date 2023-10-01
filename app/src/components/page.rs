use i18n_embed_fl::fl;
use leptos::*;

use crate::state::use_app;

#[component]
pub fn Page<F, E>(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
    #[prop(into, default = "Timada Starter App".to_owned())] title: String,
    #[prop(optional)] head: Option<F>,
) -> impl IntoView
where
    F: FnOnce() -> E + 'static,
    E: IntoView,
{
    let app = use_app();

    view! {
        <html lang=app.lang.to_owned()>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <title>{title}</title>

                <link rel="icon" href=app.create_static_url("favicon.ico")/>

                <script src="https://unpkg.com/htmx.org@1.9.6/dist/htmx.min.js" crossorigin="anonymous"></script>
                <script src="https://unpkg.com/hyperscript.org@0.9.11/dist/_hyperscript.min.js" crossorigin="anonymous"></script>

                <script>r#"htmx.on("beforeSwap",function(t){(422===t.detail.xhr.status||400===t.detail.xhr.status)&&(t.detail.shouldSwap=!0,t.detail.isError=!1)});"#</script>

                {head.map(|head| head())}
            </head>

            <body {..attrs}>
                {children()}
                <HotReload/>
            </body>
        </html>
    }
}

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let app = use_app();

    view! {
        <Page head=|| () title="404 Not Found">
            <h1>{fl!(app.fl_loader, "components_page_not-found_title")}</h1>
            <p>{fl!(app.fl_loader, "components_page_not-found_content")}</p>
            <a href=app
                .create_url("")>{fl!(app.fl_loader, "components_page_not-found_return_home")}</a>
        </Page>
    }
}

#[component]
pub fn InternalServerErrorPage() -> impl IntoView {
    let app = use_app();

    view! {
        <Page head=|| () title="500 Internal Server Error">
            <h1>{fl!(app.fl_loader, "components_page_internal-server-error_title")}</h1>
            <p>{fl!(app.fl_loader, "components_page_internal-server-error_content")}</p>
            <a href=app
                .create_url(
                    "",
                )>{fl!(app.fl_loader, "components_page_internal-server-error_return_home")}</a>
        </Page>
    }
}

#[component]
pub fn HotReload() -> impl IntoView {
    #[cfg(debug_assertions)]
    let app = use_app();

    #[cfg(debug_assertions)]
    view! {
        <script>
            {format!(
                r#"
            var es = new EventSource("{}");
            
            es.addEventListener("hot-reload", function (e) {{
                location.reload(true);
            }});
            "#,
                app.create_sse_url("/sys")
            )}

        </script>
    }

    #[cfg(not(debug_assertions))]
    None::<bool>
}
