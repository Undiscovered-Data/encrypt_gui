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

    //let mut flex21 = Flex::new(10, 10, 260, 160, None);
    //flex21.set_type(group::FlexType::Column);
    let mut label11 = frame::Frame::new(80, 8, 120, 30, "Input file");
    label11.set_label_size(30);
    let mut output = output::Output::new(50, 35, 180, 10, None);
    output.set_value("Select file");

    let mut but3 = button::Button::default()
        .with_size(80, 15)
        .with_label("Input File")
        .with_pos(100, 50);

    let mut label12 = frame::Frame::new(80, 70, 120, 30, "Output file");
    label12.set_label_size(30);
    let mut output2 = output::Output::new(50, 95, 180, 10, None);
    output2.set_value("Select folder");

    let mut but4 = button::Button::default()
        .with_size(80, 15)
        .with_label("Output Folder")
        .with_pos(100, 110);


    //flex21.end();
    win3.end();

    flex.end();
    win.make_resizable(true);
    win.end();
    win.show();

//**************************************************************\\
//*********              function callback              ********\\

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
