use std::{borrow::Cow, collections::HashMap};

use fluent::{
    types::{FluentNumber, FluentNumberOptions},
    FluentValue,
};
use poem::i18n::{I18NResources, Locale};
use tera::{Error, Filter, Result, Value};

pub struct TranslateFilter {
    locale: Locale,
}

impl TranslateFilter {
    pub fn make_for(locale: Locale) -> Self {
        Self { locale }
    }
}

impl Filter for TranslateFilter {
    fn filter(&self, id: &Value, args: &HashMap<String, Value>) -> Result<Value> {
        if args.len() == 0 {
            self.locale.text(id.as_str().unwrap())
        } else {
            let mut fluent_args = HashMap::new();
            for (key, value) in args {
                fluent_args.insert(
                    key.as_str(),
                    match value {
                        Value::Null => FluentValue::None,
                        Value::Number(val) => FluentValue::Number(FluentNumber::new(
                            val.as_f64().unwrap(),
                            FluentNumberOptions::default(),
                        )),
                        Value::String(val) => FluentValue::String(Cow::Owned(val.clone())),
                        _ => FluentValue::Error,
                    },
                );
            }
            self.locale
                .text_with_args(id.as_str().unwrap(), fluent_args)
        }
        .map(|str| Value::String(str))
        .map_err(|err| Error::msg(err))
    }
}

pub fn resources() -> I18NResources {
    I18NResources::builder()
        .add_path("web/i18n")
        .build()
        .unwrap()
}
