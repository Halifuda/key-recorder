extern crate device_query;
extern crate chrono;

use std::io::Write;
use std::fs::File;
use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct KeyRecorder<'a> {
    keycode: &'a Keycode, 
    name: String,
    counter: u32
}

fn counting() {
    let device_state = DeviceState::new();
    let k_grave = KeyRecorder {
        keycode: &Keycode::Grave,
        name: String::from("~"),
        counter: 0
    };
    let k_1 = KeyRecorder {
        keycode: &Keycode::Key1,
        name: String::from("1"),
        counter: 0
    };
    let k_2 = KeyRecorder {
        keycode: &Keycode::Key2,
        name: String::from("2"),
        counter: 0
    };
    let k_3 = KeyRecorder {
        keycode: &Keycode::Key3,
        name: String::from("3"),
        counter: 0
    };
    let k_4 = KeyRecorder {
        keycode: &Keycode::Key4,
        name: String::from("4"),
        counter: 0
    };
    let k_5 = KeyRecorder {
        keycode: &Keycode::Key5,
        name: String::from("5"),
        counter: 0
    };
    let k_6 = KeyRecorder {
        keycode: &Keycode::Key6,
        name: String::from("6"),
        counter: 0
    };
    let k_7 = KeyRecorder {
        keycode: &Keycode::Key7,
        name: String::from("7"),
        counter: 0
    };
    let k_8 = KeyRecorder {
        keycode: &Keycode::Key8,
        name: String::from("8"),
        counter: 0
    };
    let k_9 = KeyRecorder {
        keycode: &Keycode::Key9,
        name: String::from("9"),
        counter: 0
    };
    let k_0 = KeyRecorder {
        keycode: &Keycode::Key0,
        name: String::from("0"),
        counter: 0
    };
    let k_minus = KeyRecorder {
        keycode: &Keycode::Minus,
        name: String::from("-"),
        counter: 0
    };
    let k_equal = KeyRecorder {
        keycode: &Keycode::Equal,
        name: String::from("="),
        counter: 0
    };
    let k_backspace = KeyRecorder {
        keycode: &Keycode::Backspace,
        name: String::from("backspace"),
        counter: 0
    };
    let k_tab = KeyRecorder {
        keycode: &Keycode::Tab,
        name: String::from("tab"),
        counter: 0
    };    
    let k_q = KeyRecorder {
        keycode: &Keycode::Q,
        name: String::from("Q"),
        counter: 0
    };    
    let k_w = KeyRecorder {
        keycode: &Keycode::W,
        name: String::from("W"),
        counter: 0
    };    
    let k_e = KeyRecorder {
        keycode: &Keycode::E,
        name: String::from("E"),
        counter: 0
    };    
    let k_r = KeyRecorder {
        keycode: &Keycode::R,
        name: String::from("R"),
        counter: 0
    };    
    let k_t = KeyRecorder {
        keycode: &Keycode::T,
        name: String::from("T"),
        counter: 0
    };    
    let k_y = KeyRecorder {
        keycode: &Keycode::Y,
        name: String::from("Y"),
        counter: 0
    };    
    let k_u = KeyRecorder {
        keycode: &Keycode::U,
        name: String::from("U"),
        counter: 0
    };    
    let k_i = KeyRecorder {
        keycode: &Keycode::I,
        name: String::from("I"),
        counter: 0
    };    
    let k_o = KeyRecorder {
        keycode: &Keycode::O,
        name: String::from("O"),
        counter: 0
    };    
    let k_p = KeyRecorder {
        keycode: &Keycode::P,
        name: String::from("P"),
        counter: 0
    };    
    let k_lbracket = KeyRecorder {
        keycode: &Keycode::LeftBracket,
        name: String::from("Lbracket"),
        counter: 0
    };    
    let k_rbracket = KeyRecorder {
        keycode: &Keycode::RightBracket,
        name: String::from("Rbracket"),
        counter: 0
    };    
    let k_backslash = KeyRecorder {
        keycode: &Keycode::BackSlash,
        name: String::from("\\"),
        counter: 0
    };    
    let k_caps = KeyRecorder {
        keycode: &Keycode::CapsLock,
        name: String::from("capslock"),
        counter: 0
    };    
    let k_a = KeyRecorder {
        keycode: &Keycode::A,
        name: String::from("A"),
        counter: 0
    };    
    let k_s = KeyRecorder {
        keycode: &Keycode::S,
        name: String::from("S"),
        counter: 0
    };    
    let k_d = KeyRecorder {
        keycode: &Keycode::D,
        name: String::from("D"),
        counter: 0
    };    
    let k_f = KeyRecorder {
        keycode: &Keycode::F,
        name: String::from("F"),
        counter: 0
    };    
    let k_g = KeyRecorder {
        keycode: &Keycode::G,
        name: String::from("G"),
        counter: 0
    };    
    let k_h = KeyRecorder {
        keycode: &Keycode::H,
        name: String::from("H"),
        counter: 0
    };    
    let k_j = KeyRecorder {
        keycode: &Keycode::J,
        name: String::from("J"),
        counter: 0
    };    
    let k_k = KeyRecorder {
        keycode: &Keycode::K,
        name: String::from("K"),
        counter: 0
    };    
    let k_l = KeyRecorder {
        keycode: &Keycode::L,
        name: String::from("L"),
        counter: 0
    };    
    let k_semi = KeyRecorder {
        keycode: &Keycode::Semicolon,
        name: String::from(":"),
        counter: 0
    };    
    let k_apo = KeyRecorder {
        keycode: &Keycode::Apostrophe,
        name: String::from("\""),
        counter: 0
    };    
    let k_enter = KeyRecorder {
        keycode: &Keycode::Enter,
        name: String::from("enter"),
        counter: 0
    };    
    let k_lshift = KeyRecorder {
        keycode: &Keycode::LShift,
        name: String::from("Lshift"),
        counter: 0
    };  
    let k_z = KeyRecorder {
        keycode: &Keycode::Z,
        name: String::from("Z"),
        counter: 0
    };      
    let k_x = KeyRecorder {
        keycode: &Keycode::X,
        name: String::from("X"),
        counter: 0
    };       
    let k_c = KeyRecorder {
        keycode: &Keycode::C,
        name: String::from("C"),
        counter: 0
    };      
    let k_v = KeyRecorder {
        keycode: &Keycode::V,
        name: String::from("V"),
        counter: 0
    };         
    let k_b = KeyRecorder {
        keycode: &Keycode::B,
        name: String::from("B"),
        counter: 0
    };       
    let k_n = KeyRecorder {
        keycode: &Keycode::N,
        name: String::from("N"),
        counter: 0
    };       
    let k_m = KeyRecorder {
        keycode: &Keycode::M,
        name: String::from("M"),
        counter: 0
    };      
    let k_comma = KeyRecorder {
        keycode: &Keycode::Comma,
        name: String::from(","),
        counter: 0
    };       
    let k_dot = KeyRecorder {
        keycode: &Keycode::Dot,
        name: String::from("."),
        counter: 0
    };       
    let k_slash = KeyRecorder {
        keycode: &Keycode::Slash,
        name: String::from("/"),
        counter: 0
    };     
    let k_rshift = KeyRecorder {
        keycode: &Keycode::RShift,
        name: String::from("Rshift"),
        counter: 0
    };      
    let k_lctrl = KeyRecorder {
        keycode: &Keycode::LControl,
        name: String::from("Lctrl"),
        counter: 0
    };      
    let k_meta = KeyRecorder {
        keycode: &Keycode::Meta,
        name: String::from("meta"),
        counter: 0
    };      
    let k_lalt = KeyRecorder {
        keycode: &Keycode::LAlt,
        name: String::from("Lalt"),
        counter: 0
    }; 
    let k_space = KeyRecorder {
        keycode: &Keycode::Space,
        name: String::from("space"),
        counter: 0
    };      
    let k_ralt = KeyRecorder {
        keycode: &Keycode::RAlt,
        name: String::from("Ralt"),
        counter: 0
    };      
    let k_rctrl = KeyRecorder {
        keycode: &Keycode::RControl,
        name: String::from("Rctrl"),
        counter: 0
    };  
    let k_f1 = KeyRecorder {
        keycode: &Keycode::F1,
        name: String::from("F1"),
        counter: 0
    };  
    let k_f2 = KeyRecorder {
        keycode: &Keycode::F2,
        name: String::from("F2"),
        counter: 0
    };  
    let k_f3 = KeyRecorder {
        keycode: &Keycode::F3,
        name: String::from("F3"),
        counter: 0
    };  
    let k_f4 = KeyRecorder {
        keycode: &Keycode::F4,
        name: String::from("F4"),
        counter: 0
    };  
    let k_f5 = KeyRecorder {
        keycode: &Keycode::F5,
        name: String::from("F5"),
        counter: 0
    };  
    let k_f6 = KeyRecorder {
        keycode: &Keycode::F6,
        name: String::from("F6"),
        counter: 0
    };  
    let k_f7 = KeyRecorder {
        keycode: &Keycode::F7,
        name: String::from("F7"),
        counter: 0
    };  
    let k_f8 = KeyRecorder {
        keycode: &Keycode::F8,
        name: String::from("F8"),
        counter: 0
    };  
    let k_f9 = KeyRecorder {
        keycode: &Keycode::F9,
        name: String::from("F9"),
        counter: 0
    };  
    let k_f10 = KeyRecorder {
        keycode: &Keycode::F10,
        name: String::from("F10"),
        counter: 0
    };  
    let k_f11 = KeyRecorder {
        keycode: &Keycode::F11,
        name: String::from("F11"),
        counter: 0
    };  
    let k_f12 = KeyRecorder {
        keycode: &Keycode::F12,
        name: String::from("F12"),
        counter: 0
    };
    let k_insert = KeyRecorder {
        keycode: &Keycode::Insert,
        name: String::from("insert"),
        counter: 0
    };
    let k_delete = KeyRecorder {
        keycode: &Keycode::Delete,
        name: String::from("delete"),
        counter: 0
    };
    let k_home = KeyRecorder {
        keycode: &Keycode::Home,
        name: String::from("home"),
        counter: 0
    };
    let k_end = KeyRecorder {
        keycode: &Keycode::End,
        name: String::from("end"),
        counter: 0
    };
    let k_pageup = KeyRecorder {
        keycode: &Keycode::PageUp,
        name: String::from("pageup"),
        counter: 0
    };
    let k_pagedown = KeyRecorder {
        keycode: &Keycode::PageDown,
        name: String::from("pagedown"),
        counter: 0
    };
    let k_up = KeyRecorder {
        keycode: &Keycode::Up,
        name: String::from("up"),
        counter: 0
    };
    let k_down = KeyRecorder {
        keycode: &Keycode::Down,
        name: String::from("down"),
        counter: 0
    };
    let k_left = KeyRecorder {
        keycode: &Keycode::Left,
        name: String::from("left"),
        counter: 0
    };
    let k_right = KeyRecorder {
        keycode: &Keycode::Right,
        name: String::from("right"),
        counter: 0
    };

    let mut recorderlist = [
        k_grave, k_1, k_2, k_3, k_4, k_5, k_6, k_7, k_8, k_9, k_0, k_minus, k_equal,
        k_backspace, k_tab, k_q, k_w, k_e, k_r, k_t, k_y, k_u, k_i, k_o, k_p, 
        k_lbracket, k_rbracket, k_backslash, k_caps, k_a, k_s, k_d, k_f, k_g, k_h, 
        k_j, k_k, k_l, k_semi, k_apo, k_enter, k_lshift, k_z, k_x, k_c, 
        k_v, k_b, k_n, k_m, k_comma, k_dot, k_slash, k_rshift, k_lctrl, k_meta,
        k_lalt, k_space, k_ralt, k_rctrl, k_f1, k_f2, k_f3, k_f4, k_f5, k_f6, k_f7, 
        k_f8, k_f9, k_f10, k_f11, k_f12, k_insert, k_delete, k_home, k_end, 
        k_pageup, k_pagedown, k_up, k_down, k_left, k_right
    ];

    let mut prev_keys = vec![];
    let starttime: DateTime<Local> = Local::now();

    while prev_keys != [Keycode::Escape,] {
        let g_keys = device_state.get_keys();
        if g_keys != prev_keys {
            for k in &g_keys {
                for r in &mut recorderlist {
                    if k == r.keycode {
                        r.counter = r.counter + 1;
                    }
                }
            }
        }
        prev_keys = g_keys;
    }

    let endtime: DateTime<Local> = Local::now();
    let starttime = starttime.format("%Y%m%d_%H%M%S").to_string();
    let endtime = endtime.format("%Y%m%d_%H%M%S").to_string();
    let filename = "records/record-".to_owned() + &starttime + "-" + &endtime + ".txt";
    let mut recordfile = File::create(filename).unwrap();
    writeln!(recordfile, "{:#?}", starttime).expect("failed to write");
    writeln!(recordfile, "{:#?}", endtime).expect("failed to write");
    for r in &recorderlist {
        write!(recordfile, "{} ",r.counter).expect("failed to write");
    }
}

fn main() {
    std::fs::create_dir_all("./records").expect("failed to make directory");
    counting();
}