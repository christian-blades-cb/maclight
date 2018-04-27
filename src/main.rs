extern crate dbus;
#[macro_use]
extern crate error_chain;
extern crate xcb;

mod errors {
    extern crate dbus;
    extern crate xcb;
    error_chain!{
        foreign_links {
            Dbus(dbus::Error);
            TypeMismatch(dbus::arg::TypeMismatchError);
            XConnectionErr(xcb::ConnError);
            XcbError(xcb::Error<xcb::ffi::xcb_generic_error_t>);
        }
    }
}

use errors::*;
use std::io::Read;
use std::{thread, time};

static HUNDRED_MILLIS: time::Duration = time::Duration::from_millis(100);

fn main() {
    //    let (conn, _) = xcb::Connection::connect(None)?;

    // toggle_keyboard_backlight().expect("oh no");
    let first = get_light_sensor_value_apple().expect("could not fill sample buffer");
    let mut samples: [u8; 5] = [first; 5];
    let window = 5;

    thread::sleep(HUNDRED_MILLIS);
    let mut i = 0;
    loop {
        if let Ok(sample) = get_light_sensor_value_apple() {
            samples[i] = sample;
            i = (i + 1) % window;
            let avg = samples.iter().map(|&s| s as usize).sum::<usize>() / window;
            if let Err(e) = do_transition(avg as u8) {
                println!("error: {}", e);
            }
        }
        thread::sleep(HUNDRED_MILLIS);
    }
}

fn do_transition(brightness: u8) -> Result<()> {
    if brightness < 20 {
        let max = get_keyboard_max_brightness()?;
        set_keyboard_brightness(max)
    } else if brightness < 50 {
        let max = get_keyboard_max_brightness()?;
        set_keyboard_brightness(max / 2)
    } else {
        set_keyboard_brightness(0)
    }
}

fn toggle_keyboard_backlight() -> Result<()> {
    let current_brightness = get_keyboard_brightness()?;
    if current_brightness > 0 {
        set_keyboard_brightness(0)
    } else {
        let max = get_keyboard_max_brightness()?;
        set_keyboard_brightness(max)
    }
}

fn set_keyboard_brightness(brightness: i32) -> Result<()> {
    let c = dbus::Connection::get_private(dbus::BusType::System)?;
    let m = dbus::Message::new_method_call(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/KbdBacklight",
        "org.freedesktop.UPower.KbdBacklight",
        "SetBrightness",
    )?.append1(brightness);
    c.send_with_reply_and_block(m, 2000)?;
    Ok(())
}

fn get_keyboard_brightness() -> Result<i32> {
    let c = dbus::Connection::get_private(dbus::BusType::System)?;
    let m = dbus::Message::new_method_call(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/KbdBacklight",
        "org.freedesktop.UPower.KbdBacklight",
        "GetBrightness",
    )?;
    let r = c.send_with_reply_and_block(m, 2000)?;
    let brightness: i32 = r.read1()?;
    Ok(brightness)
}

fn get_keyboard_max_brightness() -> Result<i32> {
    let c = dbus::Connection::get_private(dbus::BusType::System)?;
    let m = dbus::Message::new_method_call(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/KbdBacklight",
        "org.freedesktop.UPower.KbdBacklight",
        "GetMaxBrightness",
    )?;

    let r = c.send_with_reply_and_block(m, 2000)?;
    let brightness: i32 = r.read1()?;
    Ok(brightness)
}

fn get_light_sensor_value_apple() -> Result<u8> {
    let mut fd = std::fs::File::open("/sys/devices/platform/applesmc.768/light")
        .chain_err(|| "unable to access apple light sensor")?;
    let mut foo = String::new();
    fd.read_to_string(&mut foo)
        .chain_err(|| "unable to read apple light sensor")?;
    let digits = foo.splitn(2, ",")
        .next()
        .chain_err(|| "bad response from light sensor")?;
    let digits = digits
        .get(1..)
        .chain_err(|| "bad response from light sensor")?;
    digits
        .parse::<u8>()
        .chain_err(|| "bad response from light sensor")
}

fn x_backlight_atom(conn: &xcb::base::Connection) -> Result<u32> {
    let backlight_cookie = xcb::xproto::intern_atom(&conn, true, "Backlight");
    if let Ok(reply) = backlight_cookie.get_reply() {
        return Ok(reply.atom());
    }

    let backlight_cookie = xcb::xproto::intern_atom(&conn, true, "BACKLIGHT");
    let reply = backlight_cookie.get_reply()?;

    Ok(reply.atom())
}

fn set_screen_backlight(conn: &xcb::base::Connection, atom: u32, value: i32) -> Result<()> {
    unimplemented!();
}

fn get_screen_backlight(conn: &xcb::base::Connection, atom: u32) -> Result<i32> {
    unimplemented!();
}
