extern crate device_query;
extern crate chrono;
extern crate ctrlc;

use std::io::Write;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use ctrlc::*;

static CTRLC_SIGNAL: AtomicBool = AtomicBool::new(false);

pub struct KeyRecorder<'a> {
    keycode: &'a Keycode, 
    counter: u32
}

fn counting() {
    let device_state = DeviceState::new();
    let k_grave = KeyRecorder {
        keycode: &Keycode::Grave,
        counter: 0
    };
    let k_1 = KeyRecorder {
        keycode: &Keycode::Key1,
        counter: 0
    };
    let k_2 = KeyRecorder {
        keycode: &Keycode::Key2,
        counter: 0
    };
    let k_3 = KeyRecorder {
        keycode: &Keycode::Key3,
        counter: 0
    };
    let k_4 = KeyRecorder {
        keycode: &Keycode::Key4,
        counter: 0
    };
    let k_5 = KeyRecorder {
        keycode: &Keycode::Key5,
        counter: 0
    };
    let k_6 = KeyRecorder {
        keycode: &Keycode::Key6,
        counter: 0
    };
    let k_7 = KeyRecorder {
        keycode: &Keycode::Key7,
        counter: 0
    };
    let k_8 = KeyRecorder {
        keycode: &Keycode::Key8,
        counter: 0
    };
    let k_9 = KeyRecorder {
        keycode: &Keycode::Key9,
        counter: 0
    };
    let k_0 = KeyRecorder {
        keycode: &Keycode::Key0,
        counter: 0
    };
    let k_minus = KeyRecorder {
        keycode: &Keycode::Minus,
        counter: 0
    };
    let k_equal = KeyRecorder {
        keycode: &Keycode::Equal,
        counter: 0
    };
    let k_backspace = KeyRecorder {
        keycode: &Keycode::Backspace,
        counter: 0
    };
    let k_tab = KeyRecorder {
        keycode: &Keycode::Tab,
        counter: 0
    };    
    let k_q = KeyRecorder {
        keycode: &Keycode::Q,
        counter: 0
    };    
    let k_w = KeyRecorder {
        keycode: &Keycode::W,
        counter: 0
    };    
    let k_e = KeyRecorder {
        keycode: &Keycode::E,
        counter: 0
    };    
    let k_r = KeyRecorder {
        keycode: &Keycode::R,
        counter: 0
    };    
    let k_t = KeyRecorder {
        keycode: &Keycode::T,
        counter: 0
    };    
    let k_y = KeyRecorder {
        keycode: &Keycode::Y,
        counter: 0
    };    
    let k_u = KeyRecorder {
        keycode: &Keycode::U,
        counter: 0
    };    
    let k_i = KeyRecorder {
        keycode: &Keycode::I,
        counter: 0
    };    
    let k_o = KeyRecorder {
        keycode: &Keycode::O,
        counter: 0
    };    
    let k_p = KeyRecorder {
        keycode: &Keycode::P,
        counter: 0
    };    
    let k_lbracket = KeyRecorder {
        keycode: &Keycode::LeftBracket,
        counter: 0
    };    
    let k_rbracket = KeyRecorder {
        keycode: &Keycode::RightBracket,
        counter: 0
    };    
    let k_backslash = KeyRecorder {
        keycode: &Keycode::BackSlash,
        counter: 0
    };    
    let k_caps = KeyRecorder {
        keycode: &Keycode::CapsLock,
        counter: 0
    };    
    let k_a = KeyRecorder {
        keycode: &Keycode::A,
        counter: 0
    };    
    let k_s = KeyRecorder {
        keycode: &Keycode::S,
        counter: 0
    };    
    let k_d = KeyRecorder {
        keycode: &Keycode::D,
        counter: 0
    };    
    let k_f = KeyRecorder {
        keycode: &Keycode::F,
        counter: 0
    };    
    let k_g = KeyRecorder {
        keycode: &Keycode::G,
        counter: 0
    };    
    let k_h = KeyRecorder {
        keycode: &Keycode::H,
        counter: 0
    };    
    let k_j = KeyRecorder {
        keycode: &Keycode::J,
        counter: 0
    };    
    let k_k = KeyRecorder {
        keycode: &Keycode::K,
        counter: 0
    };    
    let k_l = KeyRecorder {
        keycode: &Keycode::L,
        counter: 0
    };    
    let k_semi = KeyRecorder {
        keycode: &Keycode::Semicolon,
        counter: 0
    };    
    let k_apo = KeyRecorder {
        keycode: &Keycode::Apostrophe,
        counter: 0
    };    
    let k_enter = KeyRecorder {
        keycode: &Keycode::Enter,
        counter: 0
    };    
    let k_lshift = KeyRecorder {
        keycode: &Keycode::LShift,
        counter: 0
    };  
    let k_z = KeyRecorder {
        keycode: &Keycode::Z,
        counter: 0
    };      
    let k_x = KeyRecorder {
        keycode: &Keycode::X,
        counter: 0
    };       
    let k_c = KeyRecorder {
        keycode: &Keycode::C,
        counter: 0
    };      
    let k_v = KeyRecorder {
        keycode: &Keycode::V,
        counter: 0
    };         
    let k_b = KeyRecorder {
        keycode: &Keycode::B,
        counter: 0
    };       
    let k_n = KeyRecorder {
        keycode: &Keycode::N,
        counter: 0
    };       
    let k_m = KeyRecorder {
        keycode: &Keycode::M,
        counter: 0
    };      
    let k_comma = KeyRecorder {
        keycode: &Keycode::Comma,
        counter: 0
    };       
    let k_dot = KeyRecorder {
        keycode: &Keycode::Dot,
        counter: 0
    };       
    let k_slash = KeyRecorder {
        keycode: &Keycode::Slash,
        counter: 0
    };     
    let k_rshift = KeyRecorder {
        keycode: &Keycode::RShift,
        counter: 0
    };      
    let k_lctrl = KeyRecorder {
        keycode: &Keycode::LControl,
        counter: 0
    };      
    let k_meta = KeyRecorder {
        keycode: &Keycode::Meta,
        counter: 0
    };      
    let k_lalt = KeyRecorder {
        keycode: &Keycode::LAlt,
        counter: 0
    }; 
    let k_space = KeyRecorder {
        keycode: &Keycode::Space,
        counter: 0
    };      
    let k_ralt = KeyRecorder {
        keycode: &Keycode::RAlt,
        counter: 0
    };      
    let k_rctrl = KeyRecorder {
        keycode: &Keycode::RControl,
        counter: 0
    };  
    let k_f1 = KeyRecorder {
        keycode: &Keycode::F1,
        counter: 0
    };  
    let k_f2 = KeyRecorder {
        keycode: &Keycode::F2,
        counter: 0
    };  
    let k_f3 = KeyRecorder {
        keycode: &Keycode::F3,
        counter: 0
    };  
    let k_f4 = KeyRecorder {
        keycode: &Keycode::F4,
        counter: 0
    };  
    let k_f5 = KeyRecorder {
        keycode: &Keycode::F5,
        counter: 0
    };  
    let k_f6 = KeyRecorder {
        keycode: &Keycode::F6,
        counter: 0
    };  
    let k_f7 = KeyRecorder {
        keycode: &Keycode::F7,
        counter: 0
    };  
    let k_f8 = KeyRecorder {
        keycode: &Keycode::F8,
        counter: 0
    };  
    let k_f9 = KeyRecorder {
        keycode: &Keycode::F9,
        counter: 0
    };  
    let k_f10 = KeyRecorder {
        keycode: &Keycode::F10,
        counter: 0
    };  
    let k_f11 = KeyRecorder {
        keycode: &Keycode::F11,
        counter: 0
    };  
    let k_f12 = KeyRecorder {
        keycode: &Keycode::F12,
        counter: 0
    };
    let k_insert = KeyRecorder {
        keycode: &Keycode::Insert,
        counter: 0
    };
    let k_delete = KeyRecorder {
        keycode: &Keycode::Delete,
        counter: 0
    };
    let k_home = KeyRecorder {
        keycode: &Keycode::Home,
        counter: 0
    };
    let k_end = KeyRecorder {
        keycode: &Keycode::End,
        counter: 0
    };
    let k_pageup = KeyRecorder {
        keycode: &Keycode::PageUp,
        counter: 0
    };
    let k_pagedown = KeyRecorder {
        keycode: &Keycode::PageDown,
        counter: 0
    };
    let k_up = KeyRecorder {
        keycode: &Keycode::Up,
        counter: 0
    };
    let k_down = KeyRecorder {
        keycode: &Keycode::Down,
        counter: 0
    };
    let k_left = KeyRecorder {
        keycode: &Keycode::Left,
        counter: 0
    };
    let k_right = KeyRecorder {
        keycode: &Keycode::Right,
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
    let mut serial = vec![];
    let starttime: DateTime<Local> = Local::now();

    while CTRLC_SIGNAL.load(Ordering::Relaxed) != true {
        let g_keys = device_state.get_keys();
        if g_keys != prev_keys {
            for k in &g_keys {
                let mut i = 0;
                for r in &mut recorderlist {
                    if k == r.keycode {
                        r.counter = r.counter + 1;
                        serial.push(i);
                    }
                    i = i + 1;
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
    writeln!(recordfile, "").expect("failed to write");
    for i in serial {
        write!(recordfile, "{} ", i).expect("fail to write");
    }

    CTRLC_SIGNAL.store(false, Ordering::Relaxed);
}

fn main() {
    std::fs::create_dir_all("./records").expect("failed to make directory");
    set_handler(||{
        CTRLC_SIGNAL.store(true, Ordering::Relaxed);
    }).expect("failed to set SIGNAL handler");
    counting();
}