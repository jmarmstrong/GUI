use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Window};
const GLADE_UI: &str = include_str!("../main.glade");
use shadow_clone::shadow_clone;


fn main() {
    if gtk::init().is_err() {
        println!("GTK has failed me!");
        return;
    }
    let builder = gtk::Builder::new_from_string(GLADE_UI);
    let button: Button = builder.get_object("button").unwrap();
    let buttona: Button = builder.get_object("press").unwrap();
    let windowa: Window = builder.get_object("info1").unwrap();
    let toggle: Button = builder.get_object("buttonb").unwrap();
    let toggleb: Button = builder.get_object("buttonc").unwrap();
    let windowb: Window = builder.get_object("info2").expect("it faild here");

    {
        shadow_clone!(windowa,windowb);
        toggle.connect_clicked(move |_| {
            windowb.show_all();
            windowa.hide();
        });
    }

    {
        let windowa = windowa.clone();
        buttona.connect_clicked(move |_| windowa.clone().show_all());
    }

    windowa.connect_delete_event(|w, _| {
        w.hide();
        Inhibit(true)
    });

    {
      let windowb = windowb.clone();
       toggleb.connect_clicked(move |_| {
           windowa.show_all();
           windowb.hide();
       });
    }

    windowb.connect_delete_event(|w, _| {
        w.hide();
        Inhibit(true)
    });

    button.connect_clicked(|_| println!("Goodbye my Friend"));
    let window: Window = builder.get_object("window").unwrap();
    window.show_all();
    gtk::main();
}
