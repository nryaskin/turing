extern crate gio;
extern crate gtk;
extern crate pango;

mod turing;

#[cfg(feature = "gtk_3_10")]
mod example {

    use gio;
    use gtk;
    use pango;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{
        ApplicationWindow, Builder, Button, Dialog
    };


    use std::collections::HashMap;
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

    pub fn init_tape_dialog(builder: &gtk::Builder, main_tape_entry: gtk::Entry, machine: & Rc<RefCell<Machine>>) {
        let tape_dialog: Dialog = builder
                             .get_object("tapeDialog")
                             .expect("Couldn't get dialog for tape");
        tape_dialog.set_title("Tape Settings");

        let set_tape_button: Button = builder
                                      .get_object("buttonTapeSet")
                                      .expect("Couldn't get TapeSet button");
 
        let tape_entry: gtk::Entry = builder
                                   .get_object("tapeEntry")
                                   .expect("Couldn't get tape entry");
        
        let tape_head_spinner: gtk::SpinButton = builder
                                                 .get_object("spinbuttonTapeHead")
                                                 .expect("Couldn't get spinner");
        tape_head_spinner.set_adjustment(&gtk::Adjustment::new(0f64,0f64,100000f64, 1f64, 10f64, 5000f64));

        let tape_button_ok: Button = builder
                                     .get_object("tapeButtonOk")
                                     .expect("Couldn't get ok tape button");
        tape_button_ok.connect_clicked(clone!(tape_dialog, machine => move |_| {
            machine.borrow_mut().tape.tape = tape_entry.get_buffer().get_text().chars().collect();
            machine.borrow_mut().tape.head = 0; 
            let tape = &machine.borrow().tape;
            main_tape_entry.get_buffer().set_text(&tape.tape.iter().map(|&x| x).collect::<String>());
        
            let attr_list = pango::AttrList::new();
            let mut attr = pango::Attribute::new_background(65535, 0, 0)
                                                        .expect("Can't get background");
            attr.set_start_index(tape.head as u32);
            attr.set_end_index((tape.head + 1) as u32);
            attr_list.insert(attr);
            main_tape_entry.set_attributes(&attr_list);

            tape_dialog.hide();
        }));
        let tape_button_cancel: Button = builder
                                         .get_object("tapeButtonCancel")
                                         .expect("Couldn't get cancel button");
        tape_button_cancel.connect_clicked(clone!(tape_dialog => move |_| {
            tape_dialog.hide();
        }));

        set_tape_button.connect_clicked(clone!(tape_dialog => move |_| {
            tape_dialog.run(); 
            tape_dialog.hide();
        }));
    }

    pub fn init_rules_window(builder: &gtk::Builder, machine: &Rc<RefCell<Machine>>) {         

        let mut states = vec![];
        let mut state = HashMap::new();
        state.insert('0', Rule::Right('1',0));
        state.insert('1', Rule::Left('0',0));
        state.insert('#', Rule::Right('#',0));
        states.push(State { rules: state });


    }

    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("grid.glade");
        let builder = Builder::new_from_string(glade_src);
        let window: ApplicationWindow = builder.get_object("turingAppWindow").expect("Couldn't get window");
        window.set_application(application);
        window.set_title("Tutturu Turing Machine");

        let tape_entry: gtk::Entry = builder.get_object("entryWorkingTape").expect("Couldn't get entry working tape");
        tape_entry.set_editable(false);
        let tape_m = Tape{ tape: vec![], head: 0 };
        let machine: Rc<RefCell<Machine>> = Rc::new(RefCell::new(Machine::build_new( tape_m, vec![]))); 
    
        let main_tape_entry = tape_entry.clone();

        init_tape_dialog(&builder, main_tape_entry, &machine);
        init_rules_window(&builder, &machine);

        let button_step: Button = builder.get_object("buttonStep").expect("Couldn't get button5");
        button_step.connect_clicked(clone!(machine => move |_| {
           machine.borrow_mut().step();
           let tape = &machine.borrow().tape;
           tape_entry.get_buffer().set_text(&tape.tape.iter().map(|&x| x).collect::<String>());
        
           let attr_list = pango::AttrList::new();
           let mut attr = pango::Attribute::new_background(65535, 0, 0)
                                                        .expect("Can't get background");
           attr.set_start_index(tape.head as u32);
           attr.set_end_index((tape.head + 1) as u32);
           attr_list.insert(attr);
           tape_entry.set_attributes(&attr_list);
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
