extern crate gio;
extern crate gtk;

mod turing;

#[cfg(feature = "gtk_3_10")]
mod example {

    use gio;
    use gtk;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{
        ApplicationWindow, Builder, Button, Label, Box
    };


    use std::cell::RefCell;
    use std::rc::Rc;
    use std::env::args;

    use turing::model::*;
    use turing::Machine;

    macro_rules! clone {
        (@param _ ) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) =>({
            $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );

    }


    pub fn build_ui(application: &gtk::Application) {

        let glade_src = include_str!("grid.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder.get_object("turingAppWindow").expect("Couldn't get window");
        window.set_application(application);

        let tape_container: Box = builder.get_object("boxTape").expect("Couldn't get box tape");

        let mut labels: Vec<Label> = Vec::new();

        for i in 0..10 {
            let mut label = Label::new("1");
            let label_clone = label.clone();
            labels.push(label_clone);
            tape_container.add(&label);
        }


        let machine: Rc<RefCell<Machine>> =Rc::new(RefCell::new(Machine::build_new(Tape {
                                                  tape: vec!['r', 'u', 'r', 'r', 'u', 'r', 'u','u','u','u'],
                                                  head: 0 
                                                  },
                                                  vec![State {
                                                      rule: Rule::Right('t', 0),
                                                  }
                                                  ]
                                                  )));
let tape_t = &machine.borrow().tape;
            for i in 0..labels.len() {
                let tmp_t: String = tape_t.tape[i].to_string();
                    labels[i].set_label(&tmp_t);
            }


       let button_step: Button = builder.get_object("buttonStep").expect("Couldn't get button5");
        button_step.connect_clicked(clone!(labels, machine => move |_| {
            machine.borrow_mut().step();
            let tape = &machine.borrow().tape;
            for i in 0..labels.len() {
                let tmp: String = tape.tape[i].to_string();
                labels[i].set_label(&tmp);
            }
        }));


        window.connect_delete_event(clone!(window => move |_,_| {
            window.destroy();
            Inhibit(false)
        }));

        window.show_all();

    }

    pub fn main() {
        let application = gtk::Application::new("com.app.turringmachine",
                                        gio::ApplicationFlags::empty())
                                    .expect("Initialization failed...");
        application.connect_startup(move |app| {
            build_ui(app);
        });

        application.connect_activate( |_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}

fn main() {
    example::main()
}
