use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    I18nAssets, LanguageLoader,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub(crate) struct Localizations;

impl I18nAssets for Localizations {
    fn get_file(&self, file_path: &str) -> Option<std::borrow::Cow<'_, [u8]>> {
        Localizations::get(file_path).map(|f| f.data)
    }

    fn filenames_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(Localizations::iter().map(|f| f.to_string()))
    }
}

pub(crate) static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Load the fallback langauge by default so that users of the
    // library don't need to if they don't care about localization.
    loader
        .load_available_languages(&Localizations)
        .expect("Error while loading fallback language");

    loader
});
