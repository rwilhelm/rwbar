//extern crate xcb;

use anyhow::{anyhow, Context, Result};

use xcb;
use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};
use xcb_util::ewmh;

//use std::rc::Rc;
//use xcb_util::ewmh;

//use std::collections::HashMap;

//fn screen(&conn) -> Result(xcb::Screen<'_>> {
//    let screen = conn.get_setup().roots().nth(
//}

fn run() {

    println!("Connecting to X server");

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();


    //let ewmh_conn = ewmh::Connection::connect(conn)
    //    .map_err(|(e, _)| e)
    //    .context("Failed to wrap xcb::Connection in ewmh::Connection")?;

    let screen = conn
        .get_setup()
        .roots()
        .nth(screen_num as usize)
        .unwrap();

    let wid = conn.generate_id();

    xcb::create_window(
        &conn, /* The connection object to the server */
        xcb::COPY_FROM_PARENT as u8, // depth
        wid, // window_id
        screen.root(), // parent
        0, // x
        0, // y
        150, // width
        150, // height
        10, // border_width
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16, // class
        screen.root_visual(), // visual_id (from parent)
        &[ // value list
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (xcb::CW_EVENT_MASK,
             xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
        ]
    );

    let y = screen.height_in_pixels() - 35;

    let values = [
        (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
        (xcb::CONFIG_WINDOW_HEIGHT as u16, 35),
        (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    ];

    xcb::configure_window(&conn, wid, &values);

    xcb::map_window(&conn, wid);

    conn.flush();

    Ok(())

    //loop {
    //    let event = conn.wait_for_event();
    //    match event {
    //        None => { break; }
    //        Some(event) => {
    //            let r = event.response_type() & !0x80;
    //            match r {
    //                xcb::EXPOSE => {

    //                    /* We flush the request */
    //                    conn.flush();

    //                },
    //                xcb::KEY_PRESS => {
    //                    let key_press : &xcb::KeyPressEvent = unsafe {
    //                        xcb::cast_event(&event)
    //                    };
    //                    println!("Key '{}' pressed", key_press.detail());
    //                    break;
    //                },
    //                _ => {}
    //            }
    //        }
    //    }
    //}
}

fn main() {
    run()
}
