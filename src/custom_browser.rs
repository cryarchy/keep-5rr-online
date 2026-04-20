use anyhow::{Result, anyhow};
use headless_chrome::{Browser, Tab};
use std::sync::Arc;
use std::time::Duration;

use crate::app_config::AppConfig;

pub struct CustomBrowser {
    browser: Browser,
}

impl CustomBrowser {
    pub fn new(debug_ws_url: String, idle_browser_timeout: Duration) -> Result<Self> {
        let custom_browser = Self {
            browser: Browser::connect_with_timeout(debug_ws_url, idle_browser_timeout)?,
        };
        custom_browser.refresh()?;
        Ok(custom_browser)
    }

    pub fn get_5rr_tab(&self, config: &AppConfig) -> Result<Arc<Tab>> {
        let tabs = self.browser.get_tabs().lock().unwrap();

        let mut site_tab = None;

        for tab in tabs.iter() {
            if tab.get_url().contains(&config.site_name) && site_tab.is_none() {
                site_tab = Some(tab.clone());
            } else {
                tab.close(false)?;
            }
        }

        site_tab.ok_or(anyhow!(
            "Could not get a tab with a url containing '{}'",
            config.site_name
        ))
    }

    pub fn refresh(&self) -> Result<()> {
        let tab = &self.browser.new_tab()?;
        tab.close(false)?;
        Ok(())
    }
}
