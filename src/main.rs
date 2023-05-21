use adw::prelude::*;
use adw::{Application, gio, glib};
use adw::gtk::Builder;

const APP_ID: &str = "org.keienb.rustikal";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("org_keienb_rustikal.gresource")
        .expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let builder = Builder::from_resource("/org/keienb/rustikal/window.ui");
    let window:adw::ApplicationWindow = builder.object("app_window")
        .expect("Failed to load application window from resource");
    app.add_window(&window);
    window.present()
}