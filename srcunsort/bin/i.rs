//extern crate xcb;
//#![allow(dead_code)]
#![allow(unused_imports)]
//#![allow(unused_variables)]


mod rw {

    use anyhow::{anyhow, Context, Result};
    use std::rc::Rc;
    use xcb;
    use xcb_util::ewmh;
    use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};

    pub fn xcb_connect() {
    }

    #[derive(Clone, Debug)]
    pub enum Position {
        Top,
        Bottom
    }

    pub struct Bar {
        position: Position,
        xcb_conn: Rc<xcb::Connection>,
        screen_idx: usize,
        window_id: u32,
    }

    impl Bar {
        //pub fn new(contents: &'static str) -> Result<Bar> {
        pub fn new(position: Position) -> Result<Bar> {

            /* XCB */

            let (xcb_conn, screen_idx) =
                xcb::Connection::connect(None).context("Failed to connect to X server")?;


            //let (conn, screen_idx) = xcb::Connection::connect(None).unwrap();

            println!("xcb connected");

            let screen_idx = screen_idx as usize;
            let window_id = xcb_conn.generate_id();

            let height = 1;

            let screen = xcb_conn
                .get_setup()
                .roots()
                .nth(screen_idx)
                .ok_or_else(|| anyhow!("Invalid screen"))?;

            let values = [
                (xcb::CW_BACK_PIXEL, screen.black_pixel()),
                (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE),
            ];

            let width = screen.width_in_pixels();

            xcb::create_window(
                &xcb_conn,
                xcb::COPY_FROM_PARENT as u8,
                window_id,
                screen.root(),
                0,
                0,
                width,
                height,
                0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &values,
            );

            //let ewmh_conn = ewmh::Connection::connect(xcb_conn)
            //    .map_err(|(e, _)| e)
            //    .context("Failed to wrap xcb::Connection in ewmh::Connection")?;


            xcb::map_window(&xcb_conn, window_id);
            xcb_conn.flush(); // draws the window

            
            //println!("xcb window created");

            let bar = Bar {
                /* XCB */
                xcb_conn: Rc::new(xcb_conn),
                screen_idx,
                window_id,

                /* Bar */
                position,
            };

            Ok(bar)
        }

        pub fn poll_for_event(&self) {
            println!("polling for events");
            self.xcb_conn.poll_for_event();
        }

        pub fn wait_for_event(&self) {
            self.wait_for_event()
        }

        pub fn flush(&self) {
            self.xcb_conn.flush();
            println!("xcb connection flushed");
        }

        //fn map_window(&self) {
        //    xcb::map_window(&self.xcb_conn, self.window_id);
        //    println!("xcb window mapped");
        //}

    }
}


fn main() {
    //let bar = rw::Bar::new(rw::Position::Bottom);

    let bar = match rw::Bar::new(rw::Position::Bottom) {
        Ok(bar) => {

            match bar.poll_for_event() {
                None => {
                }
                Some(event) => {
                }
            }

            //loop {
            //    //let event = bar.wait_for_event();
            //    match event {
            //        None => { break; }
            //        Some(event) => {
            //            println!("event happened");
            //            let r = event.response_type() & !0x80;
            //            match r {
            //                xcb::EXPOSE => {

            //                    /* We flush the request */
            //                    bar.flush();

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

            //dbg!(bar.xcb_conn);
            //match bar.poll_for_event() {
            //    Some(event) => {
            //        println!("event");
            //    }
            //    None => {
            //        println!("none");
            //    }
            //}
            println!("ok");
        }
        Err(_) => {
            println!("err");
        }
    };
}








    //Ok(bar) => {
    //let mut n = 0;

    //match bar.poll_for_event() {
    //    Some(event) => {
    //    }
    //    None => {
    //    }
    //}

    //println!("bar ok");
    //loop {

    //    println!("loop {}", n);
    //    let event = bar.wait_for_event();

    //    //let event = bar.poll_for_event();

    //    match event {
    //        None => {
    //            println!("no event happened");
    //            break;
    //    //        }
    //    //        Some(event) => {
    //    //            let r = event.response_type() & !0x80;
    //    //            println!("event happened: {}", r);
    //    //            match r {
    //    //                xcb::EXPOSE => {

    //    //                    /* We flush the request */
    //    //                    println!("xcb::EXPOSE");
    //    //                    bar.flush();

    //    //                },
    //    //                xcb::KEY_PRESS => {
    //    //                    println!("xcb::KEY_PRESS");
    //    //                    let key_press : &xcb::KeyPressEvent = unsafe {
    //    //                        xcb::cast_event(&event)
    //    //                    };
    //    //                    println!("Key '{}' pressed", key_press.detail());
    //    //                    break;
    //    //                },
    //    //                _ => {
    //    //                    println!("What?"); 
    //    //                }
    //    //            }
    //    //        }
    //    //    }

    //    //    n = n + 1;
    //    //    println!("\n");

    //    //}
    //    //bar
    //}
    //Err(e) => {
    //    println!("error: {}", e);
    //    return
    //}
    //};

    //}
