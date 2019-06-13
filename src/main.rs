use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Window};
const GLADE_UI:&str = include_str!("../main.glade");

fn main() {
    if gtk::init().is_err(){
        println!("GTK has failed me!");
        return;
    }
    let builder = gtk::Builder::new_from_string(GLADE_UI);
    let button:Button = builder.get_object("button").unwrap();
    button.connect_clicked(|_|println!("Goodbye my Friend"));     
    let window:Window = builder.get_object("window").unwrap();
    window.show_all();
    gtk::main();
}

