use crate::config;
use crate::window::EpicAssetManagerWindow;
use gio::ApplicationFlags;
use glib::clone;
use glib::WeakRef;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, gio, glib};
use gtk_macros::action;
use log::{debug, info};
use once_cell::sync::OnceCell;

pub(crate) mod imp;

glib::wrapper! {
    pub struct EpicAssetManager(ObjectSubclass<imp::EpicAssetManager>)
        @extends gio::Application, gtk4::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for EpicAssetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EpicAssetManager {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(config::APP_ID)),
            ("flags", &ApplicationFlags::HANDLES_OPEN),
        ])
        .expect("Application initialization failed...")
    }

    pub fn main_window(&self) -> EpicAssetManagerWindow {
        let priv_ = crate::application::imp::EpicAssetManager::from_instance(self);
        priv_.window.get().unwrap().upgrade().unwrap()
    }

    pub fn setup_gactions(&self) {
        // Quit
        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                if let Ok(mut w) = crate::RUNNING.write() {
                    *w = false
                }
                app.main_window().close();
                app.quit();
            })
        );

        // About
        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about_dialog();
            })
        );
    }

    // Sets up keyboard shortcuts
    pub fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    pub fn setup_css(&self) {
        let provider = gtk4::CssProvider::new();
        provider.load_from_resource("/io/github/achetagames/epic_asset_manager/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk4::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk4::AboutDialogBuilder::new()
            .program_name("Epic Asset Manager")
            .logo_icon_name(config::APP_ID)
            .license_type(gtk4::License::MitX11)
            .website("https://github.com/AchetaGames/Epic-Asset-Manager")
            .version(config::VERSION)
            .transient_for(&self.main_window())
            .modal(true)
            .authors(vec!["Milan Stastny".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("Epic Asset Manager ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
