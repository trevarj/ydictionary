use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
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
        Method::Lookup { display, req } => print_lookup(client.lookup(req)?, display)?,
    }

    Ok(())
}

fn print_lookup(res: LookupResult, display: DisplayStyle) -> Result<()> {
    match display {
        DisplayStyle::Simple => print_simple(res),
        DisplayStyle::List => todo!(),
        DisplayStyle::Verbose => todo!(),
    }
}

fn print_simple(res: LookupResult) -> Result<()> {
    let out = res
        .def
        .first()
        .and_then(|e| {
            e.tr.as_ref().and_then(|tr| {
                tr.first()
                    .map(|tr| (&e.attributes.text, &tr.attributes.text))
            })
        })
        .map(|(orig, trans)| format!("{orig} - {trans}"))
        .unwrap_or_else(|| "Translation not found.".to_string());
    println!("{out}");
    Ok(())
}
