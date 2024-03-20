use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button};
use once_cell::sync::Lazy;

const APP_ID: &str = "org.gtk_rs.MainEventLoop10";

static TOKIO_RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Runtime::new().expect("Setting up tokio runtime needs to succeed.")
});

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .visible(true)
        .build();

    button.connect_clicked(move |_| {
        glib::spawn_future_local(async move {
            let response = TOKIO_RUNTIME
                .spawn(async move { reqwest::get("https://www.gtk-rs.org").await })
                .await
                .expect("future should not panic");

            if let Ok(response) = response {
                println!("Status: {}", response.status());
            } else {
                println!("Could not make a `GET` request.");
            }
        });
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
