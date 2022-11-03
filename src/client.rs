use anyhow::Result;
use ureq::Agent;
use ydictionary::methods::{GetLangsResult, LookupRequest, LookupResult};

pub struct Client {
    client: Agent,
    /// Base URL. ex. https://dictionary.yandex.net/api/v1/dicservice/
    url: String,
    /// API Key - https://yandex.com/dev/keys/
    key: String,
}

impl Client {
    pub fn new(url: &str, key: &str) -> Self {
        Self {
            client: Agent::new(),
            url: url.trim().trim_end_matches('/').to_owned(),
            key: key.to_owned(),
        }
    }

    pub fn get_langs(&self) -> Result<GetLangsResult> {
        Ok(self
            .client
            .get(&format!("{}/getLangs?key={}", self.url, self.key))
            .call()?
            .into_json()?)
    }

    pub fn lookup(&self, req: LookupRequest) -> Result<LookupResult> {
        let mut form = vec![
            ("key", self.key.as_str()),
            ("lang", req.lang.as_str()),
            ("text", req.text.as_str()),
        ];

        if let Some(ui) = &req.ui {
            form.push(("ui", ui.as_str()));
        }

        let flags;
        if let Some(val) = &req.flags {
            flags = format!("{val}");
            form.push(("flags", flags.as_str()));
        }

        Ok(self
            .client
            .get(&format!("{}/lookup", self.url))
            .send_form(form.as_slice())?
            .into_json()?)
    }
}
