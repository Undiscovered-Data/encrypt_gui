use fltk::{prelude::*, *, group::Flex, enums::Color};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(800, 500);
    win.set_color(Color::from_hex(0x444444));
    let mut flex = Flex::new(10, 10, 780, 480, None);
    flex.set_type(group::FlexType::Row);

    let mut win2 = window::Window::default().with_size(280, 180);
    win2.set_color(Color::from_hex(0x030680));
    win2.make_resizable(true);
    let mut but1 = button::Button::default()
        .with_size(100, 30)
        .with_label("Show dialog")
        .center_of_parent();

    win2.end();

    let mut win3 = window::Window::default().with_size(280, 180);
    win3.set_color(Color::from_hex(0x030680));
    win3.make_resizable(true);
    win3.end();

    flex.end();
    win.make_resizable(true);
    win.end();
    win.show();

    but1.set_callback(|_| {
        // password and input also takes a second arg which is the default value
        let pass = dialog::password_default("Enter password:", "");
        if let Some(pass) = pass {
            println!("{}", pass);
        }
    });

    a.run().unwrap();
}
