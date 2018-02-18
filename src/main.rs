
extern crate xcb;
extern crate chrono;
extern crate libc;

use chrono::prelude::*;
use xcb::ffi::xproto::xcb_change_property;
use libc::c_void;
use std::fs::File;
use std::io;
use std::io::Read;
use std::string::String;

fn file_as_number(mut file: File) -> f32 {
    let mut buf = String::new();
    file.read_to_string(&mut buf).is_ok();
    let trimmed = buf.trim_right();
    trimmed.parse::<f32>().unwrap()
}

fn get_battery(bat: &str) -> io::Result<(bool, u32)> {
    
    let mut is_discharging = false;
    let mut status = String::new();
    let mut file = File::open(format!("{}/status", bat))?;
    
    file.read_to_string(&mut status).is_ok();
    if status.trim_right() == "Discharging" {
        is_discharging = true;
    }
    
    
    let charge_full = file_as_number(File::open(
        format!("{}/charge_full", bat))?
    );
    
    let charge_now = file_as_number(File::open(
        format!("{}/charge_now", bat))?
    );

    let pcnt: f32 = charge_now / charge_full * 100.0;
    Ok((is_discharging, pcnt as u32))
}

fn get_date() -> String {
    Local::now()
        .format("%e %a, %m | %H:%M")
        .to_string()
}

fn main() {
    
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let root = setup.roots().nth(screen_num as usize).unwrap().root();
    let one_sec = std::time::Duration::new(1, 0);
    let battery =  "/sys/class/power_supply/BAT0";

    loop {

        let (is_battery_discharging, battery_pcnt) = get_battery(battery).unwrap();
        let mut battery_color = "\x01";
        if is_battery_discharging && battery_pcnt < 10 {
            battery_color = "\x03";
        }
        
        let message = format!(
            "{}{}%\x01| {}",
            battery_color,
            battery_pcnt,
            get_date()
        );

        let data = message.as_ptr() as *const c_void;

        unsafe {
            xcb_change_property(
                conn.get_raw_conn(),
                xcb::ffi::xproto::XCB_PROP_MODE_REPLACE as u8,
                root,
                xcb::ffi::xproto::XCB_ATOM_WM_NAME,
                xcb::ffi::xproto::XCB_ATOM_STRING,
                8 as u8,
                message.len() as u32,
                data
            );
        }
        
        conn.flush();
        std::thread::sleep(one_sec);
    }
}
