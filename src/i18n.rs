use std::env;
use std::sync::OnceLock;

const ENGLISH_TRANSLATIONS: &str = include_str!("../i18n/en.tsv");
const RUSSIAN_TRANSLATIONS: &str = include_str!("../i18n/ru.tsv");

#[derive(Clone, Copy)]
enum Locale {
    English,
    Russian,
}

static LOCALE: OnceLock<Locale> = OnceLock::new();

pub(crate) fn text(token: &'static str) -> &'static str {
    let catalog = match *LOCALE.get_or_init(detect_locale) {
        Locale::English => ENGLISH_TRANSLATIONS,
        Locale::Russian => RUSSIAN_TRANSLATIONS,
    };

    lookup(catalog, token).unwrap_or(token)
}

fn lookup(catalog: &'static str, token: &str) -> Option<&'static str> {
    catalog
        .lines()
        .filter_map(parse_entry)
        .find_map(|(entry_token, translation)| {
            if entry_token == token {
                Some(translation)
            } else {
                None
            }
        })
}

fn parse_entry(line: &'static str) -> Option<(&'static str, &'static str)> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    line.split_once('\t')
}

fn detect_locale() -> Locale {
    for key in ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"] {
        if env::var(key).is_ok_and(|value| contains_russian_locale(&value)) {
            return Locale::Russian;
        }
    }

    Locale::English
}

fn contains_russian_locale(value: &str) -> bool {
    value
        .split(':')
        .map(|locale| locale.trim().to_ascii_lowercase())
        .any(|locale| locale == "ru" || locale.starts_with("ru_") || locale.starts_with("ru."))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{ENGLISH_TRANSLATIONS, RUSSIAN_TRANSLATIONS, lookup, parse_entry};

    #[test]
    fn catalogs_have_the_same_tokens() {
        let english = tokens(ENGLISH_TRANSLATIONS);
        let russian = tokens(RUSSIAN_TRANSLATIONS);

        assert_eq!(english, russian);
    }

    #[test]
    fn missing_token_falls_back_to_token() {
        assert_eq!(lookup(ENGLISH_TRANSLATIONS, "unknown_token"), None);
    }

    fn tokens(catalog: &'static str) -> BTreeSet<&'static str> {
        catalog
            .lines()
            .filter_map(parse_entry)
            .map(|(token, _translation)| token)
            .collect()
    }
}
