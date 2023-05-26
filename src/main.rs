use adw::prelude::*;
use adw::{Application, ApplicationWindow, gio, glib};
use adw::gtk::Builder;

const APP_ID: &str = "org.keienb.rustikal";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("org_keienb_rustikal.gresource")
        .expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let builder = Builder::from_resource("/org/keienb/rustikal/window.ui");
    let window :ApplicationWindow = builder.object("app_window")
        .expect("Failed to load application window from resource");
    setup_actions(&window);
    app.add_window(&window);
    window.present()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.open", &["<Ctrl>o"]);
    app.set_accels_for_action("window.close", &["<Ctrl>q"]);
}

fn setup_actions(window: &ApplicationWindow) {
    let action_open = gio::SimpleAction::new("open", None);
    action_open.connect_activate(|_, _| println!("Hallo, open Welt!"));
    window.add_action(&action_open);
}
