mod app_config;
mod custom_browser;
mod error_page_detector;
mod modal_closer;

use std::{thread::sleep, time::Duration};

use anyhow::Result;
use app_config::AppConfig;
use custom_browser::CustomBrowser;
use error_page_detector::ErrorPageDetector;
use figment::{
    Figment,
    providers::{Format, Yaml},
};
use flexi_logger::Logger;
use modal_closer::ModalCloser;

static BTN_CLICK_WAIT_SECS: u64 = 1;
static PAGE_RELOAD_WAIT_SECS: u64 = 5;

fn main() -> Result<()> {
    let app_config: AppConfig = Figment::new()
        .merge(Yaml::file("app-config.yaml"))
        .extract()?;

    Logger::try_with_str(&app_config.log_level)?.start()?;

    let browser = CustomBrowser::new(
        app_config.browser_ws_url.to_owned(),
        Duration::from_secs(600),
    )?;

    let site_tab = browser.get_5rr_tab(&app_config)?;
    log::info!("Tab title: {}", site_tab.get_title()?);

    ModalCloser::close_open_modal(&site_tab)?;

    loop {
        while ErrorPageDetector::process(&site_tab)? {}

        let val = rand::random_range(0..app_config.links_to_shuffle.len());
        let url = &app_config.links_to_shuffle[val];
        log::info!("Navigate to: {url}");
        site_tab.navigate_to(url)?;

        let mins = rand::random_range(0..=5);
        log::info!("Sleep for {mins} minutes");
        sleep(Duration::from_secs(60 * mins));
    }
}
