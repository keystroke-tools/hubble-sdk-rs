use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
/// This represents all languages supported by Postgres.
pub enum Language {
    Simple,
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Nepali,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Yiddish,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang_str = match self {
            Language::Simple => "Simple",
            Language::Arabic => "Arabic",
            Language::Armenian => "Armenian",
            Language::Basque => "Basque",
            Language::Catalan => "Catalan",
            Language::Danish => "danish",
            Language::Dutch => "dutch",
            Language::English => "english",
            Language::Finnish => "finnish",
            Language::French => "french",
            Language::German => "german",
            Language::Greek => "greek",
            Language::Hindi => "hindi",
            Language::Hungarian => "hungarian",
            Language::Indonesian => "indonesian",
            Language::Irish => "irish",
            Language::Italian => "italian",
            Language::Lithuanian => "lithuanian",
            Language::Nepali => "nepali",
            Language::Norwegian => "norwegian",
            Language::Portuguese => "portuguese",
            Language::Romanian => "romanian",
            Language::Russian => "russian",
            Language::Serbian => "serbian",
            Language::Spanish => "spanish",
            Language::Swedish => "swedish",
            Language::Tamil => "tamil",
            Language::Turkish => "turkish",
            Language::Yiddish => "yiddish",
        };

        f.write_str(lang_str)
    }
}

/// This is used to infer the Postgres-compatible language from a given text.
pub fn detect_lang(text: &str) -> Language {
    let lang = whatlang::detect_lang(text);
    if lang.is_none() {
        return Language::Simple;
    }

    let lang = lang.unwrap();

    match lang {
        whatlang::Lang::Eng => Language::English,
        whatlang::Lang::Rus => Language::Russian,
        whatlang::Lang::Spa => Language::Spanish,
        whatlang::Lang::Por => Language::Portuguese,
        whatlang::Lang::Ita => Language::Italian,
        whatlang::Lang::Fra => Language::French,
        whatlang::Lang::Deu => Language::German,
        whatlang::Lang::Ara => Language::Arabic,
        whatlang::Lang::Hin => Language::Hindi,
        whatlang::Lang::Yid => Language::Yiddish,
        whatlang::Lang::Dan => Language::Danish,
        whatlang::Lang::Swe => Language::Swedish,
        whatlang::Lang::Fin => Language::Finnish,
        whatlang::Lang::Tur => Language::Turkish,
        whatlang::Lang::Hun => Language::Hungarian,
        whatlang::Lang::Ell => Language::Greek,
        whatlang::Lang::Ron => Language::Romanian,
        whatlang::Lang::Srp => Language::Serbian,
        whatlang::Lang::Lit => Language::Lithuanian,
        whatlang::Lang::Tam => Language::Tamil,
        whatlang::Lang::Ind => Language::Indonesian,
        whatlang::Lang::Nep => Language::Nepali,
        whatlang::Lang::Cat => Language::Catalan,
        whatlang::Lang::Hye => Language::Armenian,
        _ => Language::Simple,
    }
}
