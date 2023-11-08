use adw::prelude::GtkWindowExt;
use gtk::{
    gio, glib,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use window::Window;

mod other_page;
mod window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gtkadw.gresource").expect("Failed to register resources.");

    let app = adw::Application::builder()
        .application_id("se.andras.gtkadw")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}
