use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use rsmorphy::dict_ru::DICT_PATH;
use rsmorphy::{MorphAnalyzer, Source};
use serde::Deserialize;
use ydictionary::methods::{LookupRequest, LookupResult};

use crate::client::Client;

mod client;

/// A tool to lookup a word on Yandex Dictionary. Powered by Yandex.Dictionary https://tech.yandex.com/dictionary.
#[derive(Parser)]
struct Args {
    /// Yandex Dictionary API Key
    key: Option<String>,
    #[clap(subcommand)]
    method: Method,
}

#[derive(Debug, Subcommand)]
enum Method {
    /// Get all supported languages.
    Langs,
    /// Lookup a word.
    Lookup {
        #[clap(short, value_enum, default_value_t)]
        display: DisplayStyle,
        #[clap(flatten)]
        req: LookupRequest,
    },
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
enum DisplayStyle {
    /// Display one word translation.
    #[default]
    Simple,
    /// List all translations.
    List,
    /// List all translations with things like examples.
    Verbose,
}

#[derive(Debug, Default, Deserialize)]
struct Config {
    /// API Key
    key: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // check .config for API key
    let config = match dirs::config_dir().map(|d| std::fs::read(d.join("ydictionary/config.toml")))
    {
        Some(Ok(bytes)) => toml::from_slice::<Config>(&bytes)?,
        Some(Err(e)) if e.kind() == std::io::ErrorKind::NotFound => Config::default(),
        Some(Err(e)) => bail!("Couldn't open config: {e:?}"),
        None => Config::default(),
    };

    let key = args
        .key
        .or(config.key)
        .ok_or_else(|| anyhow!("No API Key provided."))?;

    let client = Client::new("https://dictionary.yandex.net/api/v1/dicservice.json", &key);

    match args.method {
        Method::Langs => println!("{:?}", client.get_langs()?),
        Method::Lookup { display, req } => print_lookup(client.lookup(maybe_morph(req))?, display)?,
    }

    Ok(())
}

fn maybe_morph(mut req: LookupRequest) -> LookupRequest {
    if req.lang == "ru-en" {
        let morph = MorphAnalyzer::from_file(DICT_PATH);
        let normal = morph
            .parse(&req.text)
            .first()
            .map(|w| w.lex.get_normal_form(&morph).to_string())
            .unwrap_or(req.text);
        req.text = normal;
    }
    req
}

fn print_lookup(res: LookupResult, display: DisplayStyle) -> Result<()> {
    let out = match display {
        DisplayStyle::Simple => print_simple(res),
        DisplayStyle::List => todo!(),
        DisplayStyle::Verbose => todo!(),
    }
    .ok_or_else(|| anyhow!("Translation not found"))?;
    println!("{out}");
    Ok(())
}

fn print_simple(res: LookupResult) -> Option<String> {
    res.def
        .first()
        .and_then(|e| {
            e.tr.as_ref().and_then(|tr| {
                tr.first()
                    .map(|tr| (&e.attributes.text, &tr.attributes.text))
            })
        })
        .map(|(orig, trans)| format!("{orig} - {trans}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morphing() {
        assert_eq!(maybe_morph(LookupRequest::ru_en("твою")).text, "твой");
        assert_eq!(maybe_morph(LookupRequest::ru_en("ищу")).text, "искать");
        assert_eq!(maybe_morph(LookupRequest::ru_en("яблоко")).text, "яблоко");
        assert_eq!(maybe_morph(LookupRequest::en_ru("friend")).text, "friend");
    }
}
