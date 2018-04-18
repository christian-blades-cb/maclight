#[macro_use]
extern crate error_chain;
extern crate dbus;

mod errors {
    extern crate dbus;
    error_chain!{
        foreign_links {
            Dbus(dbus::Error);
            TypeMismatch(dbus::arg::TypeMismatchError);
        }
    }
}

use errors::*;

fn main() {
    let current_brightness = get_keyboard_brightness().expect("boo");
    println!("brightness: {}", &current_brightness);
    if current_brightness > 0 {
        set_keyboard_brightness(0).expect("oh nooooo");
    } else {
        set_keyboard_brightness(255).expect("oh nooooo");
    }
}

fn set_keyboard_brightness(brightness: i32) -> Result<()>{
    let c = dbus::Connection::get_private(dbus::BusType::System)?;
    let m = dbus::Message::new_method_call("org.freedesktop.UPower", "/org/freedesktop/UPower/KbdBacklight", "org.freedesktop.UPower.KbdBacklight", "SetBrightness")?.append1(brightness);
    c.send_with_reply_and_block(m, 2000)?;
    Ok(())
}

fn get_keyboard_brightness() -> Result<i32> {
    let c = dbus::Connection::get_private(dbus::BusType::System)?;
    let m = dbus::Message::new_method_call("org.freedesktop.UPower", "/org/freedesktop/UPower/KbdBacklight", "org.freedesktop.UPower.KbdBacklight", "GetBrightness")?;
    let r = c.send_with_reply_and_block(m, 2000)?;
    let brightness: i32 = r.read1()?;
    Ok(brightness)
}
