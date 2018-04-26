# maclight [![Build Status](https://travis-ci.org/christian-blades-cb/maclight.svg?branch=master)](https://travis-ci.org/christian-blades-cb/maclight)

Automatic keyboard backlight control for macbooks.

You want to run Linux on your mac hardware, but you still want the backlight value changed according to ambient light?

maclight is your solution!

## How does it work?

Check out `get_light_sensor_value_apple` to see what device in `/sys` is read to get the ambient light value.

DBus is used to find the keyboard brightness range.

At some point soon, the screen backlight will also change according to the light sensor.
