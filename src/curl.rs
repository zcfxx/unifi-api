use std::env::var;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Sites {
    host_id: String,
    site_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Body {
    sites: [Sites; 1],
}

impl Body {
    pub fn new() -> Self {
        Self {
            sites: [Sites {
                host_id: String::new(),
                site_id: String::new(),
            }],
        }
    }

    pub fn with_host(&mut self, host_id: impl Into<String>) -> &mut Self {
        self.sites[0].host_id = host_id.into();
        self
    }

    pub fn with_site(&mut self, site_id: impl Into<String>) -> &mut Self {
        self.sites[0].site_id = site_id.into();
        self
    }
}

pub struct Curl {
    host: String,
}

impl Curl {
    // let list_host_url: String = var("URL_LIST_HOSTS").unwrap();

    pub fn new() -> Self {
        Self {
            host: String::new(),
        }
    }

    pub fn with_list_hosts(&mut self) -> &mut Self {
        self.host = var("URL_LIST_HOSTS").unwrap();
        self
    }
}
