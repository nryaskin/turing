#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate gio;
extern crate gtk;
extern crate pango;

mod turing;

#[cfg(feature = "gtk_3_10")]
mod example {

    use std::collections::HashSet;
    #[derive(Serialize, Deserialize)]
    struct GridHelper {
        chars: Vec<char>,
        state_count: i32
    }

    use serde_yaml;
    use gio;
    use gtk;
    use pango;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{
        ApplicationWindow, Builder, Button, Dialog, Window, ComboBoxText, WindowType, Menu, FileChooserDialog, ResponseType
    };
    use gio::MenuExt;

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
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

    fn init_tape_dialog(builder: &gtk::Builder, main_tape_entry: gtk::Entry, machine: & Rc<RefCell<Machine>>) {
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

    fn init_add_char_dialog(builder: &gtk::Builder, rules_grid: &gtk::Grid, rules: &Rc<RefCell<GridHelper>>) {
        let char_set_dialog: Dialog = builder.get_object("charSelectDialog")
            .expect("Couldn't get dialog set char");
        let button_set_char: Button = builder.get_object("newCharButton")
            .expect("Couldn't get button set char");
        button_set_char.connect_clicked(clone!(char_set_dialog => move |_| {
            char_set_dialog.run();
            char_set_dialog.hide();
        }));

        let char_entry: gtk::Entry = builder.get_object("charEntry")
            .expect("Couldn't get entry for char dialog");
        let button_ok_char: Button = builder.get_object("charOkButton")
            .expect("Couldn't get ok button for char dialog");

        button_ok_char.connect_clicked(clone!(char_entry, rules, rules_grid, char_set_dialog => move |_| {
            let c = char_entry.get_buffer().get_text().chars().next().expect("");
            if !rules.borrow().chars.contains(&c) {
                rules.borrow_mut().chars.push(c);
                let e: gtk::Label = gtk::Label::new("");
                e.set_label(&String::from(c.to_string()));
                rules_grid.attach(&e, 0, rules.borrow().chars.len() as i32, 1, 1);
                rules_grid.show_all();
                char_set_dialog.hide();
            }
        }));

        let button_cancel_char: Button = builder.get_object("cancelCharButton")
            .expect("Couldn't get cancel button for char dialog");
        button_cancel_char.connect_clicked(clone!(char_set_dialog => move |_| {
            char_set_dialog.hide();
        }));
    }

    fn init_rules_window(builder: &gtk::Builder, machine: &Rc<RefCell<Machine>>) {         

        let grid_helper = Rc::new(RefCell::new(GridHelper { chars: vec![], state_count: 0 }));
        let rule_window: gtk::Window = builder.get_object("rulesWindow")
            .expect("Couldn't get window");
        rule_window.set_title("Rules Set Window");

        let rules_grid: gtk::Grid = builder.get_object("rulesGrid")
            .expect("couldn't get rules grid");
        let dummy = gtk::Label::new("symbol\\state");
        rules_grid.attach(&dummy, 0, 0, 1, 1);

        let button_add_state: Button = builder.get_object("addRuleButton")
            .expect("Couldn't get add state button");
        
        button_add_state.connect_clicked(clone!(rules_grid, grid_helper, machine => move |_| {
            let t = grid_helper.borrow().state_count + 1;
            grid_helper.borrow_mut().state_count = t;
            rules_grid.insert_column(t);
            let lab: gtk::Label = gtk::Label::new("");
            lab.set_label(&(String::from("q") + &t.to_string()));
            rules_grid.attach(&lab, t , 0, 1, 1);
            for i in 0..grid_helper.borrow().chars.len() {
                let b = Button::new();
                b.set_label("+");
                b.connect_clicked(clone!(machine, grid_helper, b => move |_| {
                    if machine.borrow().states.len() < t as usize {
                        let index =  machine.borrow().states.len();
                        for _j in index..(t as usize) {
                            machine.borrow_mut().states.push(State { rules: HashMap::new()});
                        }
                    }

                       let d = init_rule_dialog_chooser(&grid_helper, &machine, (t - 1) as usize, grid_helper.borrow().chars[i], &b);
                        //machine.borrow_mut().states[(t - 1) as usize].rules.insert(grid_helper.borrow().chars[i],Rule::Right('p', 0));
                       d.show_all();

                    
                }));
                rules_grid.attach(&b, t, i as i32 + 1, 1, 1);
                 
            }
            rules_grid.show_all();
        }));
 
        init_add_char_dialog(&builder, &rules_grid, &grid_helper);
        

        let button_set_rules: Button = builder.get_object("buttonSetRules")
            .expect("Couldn't get button set rules");
        button_set_rules.connect_clicked(clone!(rule_window => move |_| {
            rule_window.fullscreen();
            rule_window.show_all(); 
        }));

        let button_ok: Button = builder.get_object("okButton")
            .expect("Couldn't get ok button");
        button_ok.connect_clicked(clone!(rule_window => move |_| {
            rule_window.destroy();
            Inhibit(false);
        }));
        let button_cancel: Button = builder.get_object("cancelButton")
            .expect("Couldn't get cancel button");

        button_cancel.connect_clicked(clone!( rule_window => move |_| {
            rule_window.destroy();
        }));
        rule_window.connect_delete_event(clone!(rule_window => move |_,_| {
            rule_window.destroy();
            Inhibit(false)
        }));
    }


    fn init_rule_dialog_chooser(grid_helper: &Rc<RefCell<GridHelper>>, machine: &Rc<RefCell<Machine>>,i: usize, c: char, b: &gtk::Button) -> gtk::Window {
        let dialog_window: gtk::Window = gtk::Window::new(WindowType::Popup);
        let content_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 100);
        let combowombo = ComboBoxText::new();
        for c_items in &grid_helper.borrow().chars {
            combowombo.append_text(&c_items.to_string());
        }
        let ch_sel = combowombo.clone();
        content_box.add(&combowombo);
        let combowombo = ComboBoxText::new();
        combowombo.append_text("left");
        combowombo.append_text("right");
        let rule_sel = combowombo.clone();
        content_box.add(&combowombo);
        let combowombo = ComboBoxText::new();
        for i in 0..grid_helper.borrow().state_count{
            combowombo.append_text(&i.to_string());
        }
        let state_sel = combowombo.clone();
        content_box.add(&combowombo);

        let butt = Button::new();
        butt.set_label("ok");
        butt.connect_clicked(clone!(dialog_window, ch_sel, rule_sel, state_sel, machine, b => move |_| {
            let rule_text = &rule_sel;
            let rule;
            let but_name;
            let ch = ch_sel.get_active_text().unwrap().chars().next().unwrap();
            let state = state_sel.get_active_text().unwrap().parse::<usize>().unwrap();
            if rule_text.get_active_text() == Some(String::from("left")) {
                rule = Rule::Left(ch, state);
                but_name = ch.to_string() + &String::from("l") + &state.to_string();
            }
            else {
                rule = Rule::Right(ch, state);
                but_name = ch.to_string() + &String::from("r") + &state.to_string();
            }
            machine.borrow_mut().states[i].rules.insert(c, rule);
            b.set_label(&but_name);
            dialog_window.destroy();
        }));
        content_box.add(&butt);
        dialog_window.add(&content_box);
        
        return dialog_window;
    }

    fn init_menu_items(application: &gtk::Application) {
        let menu_bar = gio::Menu::new();
        let file_menu = gio::Menu::new(); 
        file_menu.append("_Open", "app.open");

        file_menu.append("_Save", "app.save");

        menu_bar.append_submenu("_File", &file_menu);
        application.set_app_menu(&menu_bar);
    }

    fn deserialize(file: & mut File, machine: &Rc<RefCell<Machine>>) {
        let mut buf_reader = BufReader::new(file);
        let mut out_str = String::new();
        buf_reader.read_to_string(& mut out_str);
        let deserialize: Machine = serde_yaml::from_str(&out_str).unwrap();
        machine.borrow_mut().tape = deserialize.tape;
        machine.borrow_mut().states = deserialize.states;
        machine.borrow_mut().current_state = deserialize.current_state;
    }

    fn add_actions(application: &gtk::Application, machine: &Rc<RefCell<Machine>>, tape: gtk::Entry) {
        let open_action = gio::SimpleAction::new("open", None);
        open_action.connect_activate(clone!(machine => move |_,_| {
            let file_open = gtk::FileChooserDialog::new(Some("Open machine"), Some(&Window::new(WindowType::Popup)), gtk::FileChooserAction::Open);
            file_open.add_button("Cancel", 0);
            file_open.add_button("Ok", 1); //ResponseType into issue
            if 1 == file_open.run() {
                if let Some(op_file) = file_open.get_filename(){
                    if let Ok(mut file) = File::open(&op_file) {
                        deserialize(& mut file, &machine);
                    }
                }
            }
            file_open.destroy();
        }));
        let save_action = gio::SimpleAction::new("save", None);
        save_action.connect_activate(clone!(machine => move |_,_| {
            let file_save = gtk::FileChooserDialog::new(Some("Save machine"), Some(&Window::new(WindowType::Popup)), gtk::FileChooserAction::Save);
            file_save.add_button("Cancel", 0);
            file_save.add_button("Ok", 1);
            if file_save.run() == 1 {
                if let Some(sv_file) = file_save.get_filename() {
                    if let Ok(mut file) = File::create(sv_file){
                        let replacement = Machine{tape: machine.borrow().tape.clone(), states: machine.borrow().states.clone(), current_state: machine.borrow().current_state};
                        let serialized = serde_yaml::to_string(&replacement).unwrap();
                        file.write_all(serialized.as_bytes()); 
                    }
                }
            }
            file_save.destroy();
        }));
        application.add_action(&open_action);
        application.add_action(&save_action);
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
        init_menu_items(&application);
        let main_tape_entry = tape_entry.clone();
        add_actions(&application, &machine, main_tape_entry);
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
