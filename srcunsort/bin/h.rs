//#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![allow(unused_imports)]


mod rw {

    use anyhow::{anyhow, Context, Result};
    use std::rc::Rc;
    use xcb;
    use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};
    use xcb_util::ewmh;

    fn get_root_visual_type(conn: &xcb::Connection, screen: &xcb::Screen<'_>) -> xcb::Visualtype {
        for root in conn.get_setup().roots() {
            for allowed_depth in root.allowed_depths() {
                for visual in allowed_depth.visuals() {
                    if visual.visual_id() == screen.root_visual() {
                        return visual;
                    }
                }
            }
        }
        panic!("No visual type found");
    }

    pub struct ComputedText {
    }

    impl ComputedText {
        fn render() {
        }
    }

    #[derive(Clone, Debug)]
    pub enum Position {
        Top,
        Bottom
    }

    pub struct Bar {
        position: Position,

        pub conn: Rc<ewmh::Connection>,
        screen_idx: usize,
        pub window_id: u32,

        surface: cairo::XCBSurface,
        width: u16,
        height: u16,

        contents: Vec<Vec<ComputedText>>,
    }

    impl Bar {
        //pub fn new(contents: &'static str) -> Result<Bar> {
        pub fn new(position: Position) -> Result<Bar> {

            /* XCB */

            let (conn, screen_idx) =
                xcb::Connection::connect(None).context("Failed to connect to X server")?;

            println!("xcb connected");

            let screen_idx = screen_idx as usize;
            let window_id = conn.generate_id();

            let height = 1;

            let screen = conn
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
                &conn,
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

            println!("xcb window created");







            let y = screen.height_in_pixels() - 35;

            // Update the height/position of the XCB window and the height of the Cairo surface.

            let values = [
                (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, 35),
                (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
            ];

            xcb::configure_window(&conn, window_id, &values);

            //conn.flush();

            //xcb::map_window(&conn, wid);
            xcb::map_window(&conn, window_id);

            //conn.flush();






            /* Cairo */

            let cairo_conn = unsafe {
                cairo::XCBConnection::from_raw_none(conn.get_raw_conn() as *mut cairo_sys::xcb_connection_t)
            };

            println!("cairo connected");

            let visual = unsafe {
                cairo::XCBVisualType::from_raw_none(
                    &mut get_root_visual_type(&conn, &screen).base as *mut xcb::ffi::xcb_visualtype_t
                    as *mut cairo_sys::xcb_visualtype_t,
                )
            };

            let drawable = cairo::XCBDrawable(window_id);

            let surface = cairo::XCBSurface::create(&cairo_conn, &drawable, &visual, i32::from(width), i32::from(height))
                .map_err(|status| anyhow!("XCBSurface::create: {}", status))?;

            println!("cairo surface created");

            /* EWMH */

            let ewmh_conn = ewmh::Connection::connect(conn)
                .map_err(|(e, _)| e)
                .context("Failed to wrap xcb::Connection in ewmh::Connection")?;

            println!("ewmh connected");

            //conn.flush();

            /* Bar Object */

            let bar = Bar {
                /* XCB */
                conn: Rc::new(ewmh_conn),
                screen_idx,
                window_id,

                /* Cairo */
                surface,
                height,
                width,

                /* Bar */
                position,
                contents: Vec::new()
            };


            Ok(bar)
        }

        pub fn flush(&self) {
            self.conn.flush();
            println!("xcb connection flushed");
        }

        fn map_window(&self) {
            xcb::map_window(&self.conn, self.window_id);
            println!("xcb window mapped");
        }

        pub fn connection(&self) -> &Rc<ewmh::Connection> {
            &self.conn
        }
        
        // Process an X event received from the `Bar::connection()`.
        pub fn process_event(&mut self, event: xcb::GenericEvent) -> Result<()> {
            let expose = event.response_type() & !0x80 == xcb::EXPOSE;
            if expose {
                println!("Redrawing entire bar - expose event.");
                //self.redraw_entire_bar()?;
            }
            Ok(())
        }

        fn redraw_content(&mut self, idx: usize) -> Result<()> {
            //for text in &mut self.contents[idx] {
            //    text.render(&self.surface)?;
            //}

            self.flush();

            Ok(())
        }

        pub fn redraw_entire_bar(&mut self) -> Result<()> {
            //self.recompute_dimensions()?;

            for idx in 0..self.contents.len() {
                self.redraw_content(idx)?;
            }
            Ok(())
        }




    }
}

fn main() {
    let bar = match rw::Bar::new(rw::Position::Bottom) {
        Ok(bar) => {
            println!("bar initialized");
            bar
        }
        Err(e) => {
            println!("error: {}", e);
            return
        }
    };


    loop {
        let event = bar.conn.wait_for_event();

        println!("listening to events");

        match event {
            None => {
                println!("no event happened");
                break;
            }
            Some(event) => {
                let r = event.response_type() & !0x80;
                println!("event happened: {}", r);
                match r {
                    xcb::EXPOSE => {

                        /* We flush the request */
                        println!("xcb::EXPOSE");
                        bar.conn.flush();

                    },
                    xcb::KEY_PRESS => {
                        println!("xcb::KEY_PRESS");
                        let key_press : &xcb::KeyPressEvent = unsafe {
                            xcb::cast_event(&event)
                        };
                        println!("Key '{}' pressed", key_press.detail());
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    //println!("{}", bar.height);

    //let t = match rw::Bar::new("asdf") {
    //    Ok(bar) => bar,
    //    Err(_) => return 
    //};

}
