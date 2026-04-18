use super::PAGE_RELOAD_WAIT_SECS;
use anyhow::Result;
use headless_chrome::Tab;
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

pub struct ErrorPageDetector {}

impl ErrorPageDetector {
    pub fn error_code_selector() -> &'static str {
        "body > main > article > code"
    }

    fn figcaption_selector() -> &'static str {
        "body > main > figure > figcaption > h1"
    }

    pub fn is_error_page(tab: &Arc<Tab>) -> Result<bool> {
        let element_selector = Self::error_code_selector();
        log::info!("Find element: {element_selector}");
        if let Ok(code_el) = tab.find_element(element_selector) {
            log::info!("Get inner text: {element_selector}");
            let text = code_el.get_inner_text()?;
            if text.contains("ERRCODE") {
                return Ok(true);
            }
        }

        let element_selector = Self::figcaption_selector();
        log::info!("Find element: {element_selector}");
        if let Ok(caption_el) = tab.find_element(element_selector) {
            log::info!("Get inner text: {element_selector}");
            let text = caption_el.get_inner_text()?;
            if text.contains("no time") {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn process(tab: &Arc<Tab>) -> Result<bool> {
        let element_selector = Self::error_code_selector();
        log::info!("Find element: {element_selector}");
        if Self::is_error_page(tab)? {
            log::info!("Reload tab");
            tab.reload(true, None)?;
            sleep(Duration::from_secs(PAGE_RELOAD_WAIT_SECS));
            return Ok(true);
        }
        Ok(false)
    }
}
