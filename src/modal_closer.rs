use super::BTN_CLICK_WAIT_SECS;
use anyhow::Result;
use headless_chrome::Tab;
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

pub struct ModalCloser {}

impl ModalCloser {
    pub fn open_modal_close_btn_selector() -> &'static str {
        r#"article[aria-modal="true"][role="dialog"] button:has(svg)"#
    }

    pub fn close_open_modal(tab: &Arc<Tab>) -> Result<()> {
        let element_selector = Self::open_modal_close_btn_selector();
        log::info!("Find element: {element_selector}");
        if let Ok(modal_close_btn) = tab.find_element(element_selector) {
            log::info!("Click: {element_selector}");
            modal_close_btn.click()?;
            sleep(Duration::from_secs(BTN_CLICK_WAIT_SECS));
        }
        Ok(())
    }
}
