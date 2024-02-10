use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, Language};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        locales: LOCALES,
        languages: "./locales/languages.json",
        provide_context: true,
    }};

    view! { <OtherComponent/> }
}

#[component]
fn OtherComponent() -> impl IntoView {
    let i18n_ctx = i18n();

    view! {
        <p>{move || i18n().tr("select-a-language")}</p>
        <fieldset>
            <For
                each=move || i18n().languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang.id.to_string()
                                name="language"
                                value=lang.id.to_string()
                                checked=*lang == i18n_ctx.language.get()
                                on:click=move |_| i18n().language.set(lang)
                            />
                            <label for=lang.id.to_string()>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
    }
}
