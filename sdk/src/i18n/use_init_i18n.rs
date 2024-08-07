use dioxus::prelude::*;
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;

use super::use_i18n::Language;

pub struct UseInitI18Data {
    pub(crate) fallback_language: FallbackLanguage,
    pub(crate) languages: HashMap<LanguageIdentifier, Language>,
}

pub(crate) struct FallbackLanguageMap {
    pub(crate) default: LanguageIdentifier,
    pub(crate) map: HashMap<LanguageIdentifier, Vec<LanguageIdentifier>>,
}

pub enum FallbackLanguage {
    Single(LanguageIdentifier),
    Map(FallbackLanguageMap),
}


pub fn use_init_i18n(
    selected_language: LanguageIdentifier,
    fallback_language: FallbackLanguage,
    languages: impl FnOnce() -> Vec<Language>,
) {
    let selected_language = use_signal(|| selected_language);

    let languages_vec = languages();
    let languages_map = languages_vec.into_iter()
        .map(|language| (language.id.clone(), language))  // Clone or copy id as needed
        .collect::<HashMap<_, _>>();

    let init_i18_data = use_signal(|| UseInitI18Data {
        languages: languages_map,
        fallback_language: fallback_language,
    });

    provide_context(selected_language);
    provide_context(init_i18_data);
}
