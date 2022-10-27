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
        .with_size(80, 30)
        .with_label("Set Password")
        .with_pos(100, 30);

    let mut frame1 = frame::Frame::new(75, 60, 110, 70, "> Encrypt");
    frame1.set_label_size(30);

    let mut but2 = button::Button::default()
        .with_size(80, 30)
        .with_label("Encrypt\nor\nDecrypt")
        .with_pos(100, 120);
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

    let mut a_state: u8 = 0;
    but2.set_callback(move |_| {
        if a_state == 0 {
            frame1.set_label("< Decrypt");
            a_state += 1;
        }
        else {
            frame1.set_label("> Encrypt");
            a_state -= 1;
        }
    });

    a.run().unwrap();
}
