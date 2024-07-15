use std::path::PathBuf;

use reqwest::{cookie::CookieStore, header::HeaderValue};

use crate::{config::get_project_os_directory, errors::Result};

pub struct HnSecureCookiesStore {}

impl HnSecureCookiesStore {
    fn refresh_from_disk(&mut self) -> Result<()> {
        Ok(())
    }

    fn save_to_disk(&self) -> Result<()> {
        Ok(())
    }

    fn get_cookies_file_path() -> Result<PathBuf> {
        get_project_os_directory().map(|directory| directory.join("secure"))
    }
}

impl CookieStore for HnSecureCookiesStore {
    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        None
    }

    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        for header in cookie_headers {}
    }
}
