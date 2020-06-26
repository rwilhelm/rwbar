//#![allow(dead_code)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]

mod rw {

    use anyhow::{anyhow, Context, Result};
    use std::rc::Rc;
    use xcb;
    use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};
    use xcb_util::ewmh;
    
    use tokio::runtime::Runtime;
    use tokio::stream::{StreamExt, StreamMap};
    use tokio::task;

    //pub fn run(self) -> Result<()> {
    //    // Use a single-threaded event loop. We aren't interested in
    //    // performance too much, so don't mind if we block the loop
    //    // occasionally. We are using events to get woken up as
    //    // infrequently as possible (to save battery).
    //    let mut rt = Runtime::new()?;
    //    let local = task::LocalSet::new();
    //    local.block_on(&mut rt, self.run_inner())?;
    //    Ok(())
    //}

    fn create_window(
        conn: &xcb::Connection,
        screen_idx: usize,
        window_id: u32,
        height: u16,
    ) -> Result<(u16, cairo::XCBSurface)> {

        println!("> create_window");

        let screen = conn
            .get_setup()
            .roots()
            .nth(screen_idx)
            .ok_or_else(|| anyhow!("Invalid screen"))?;

        let width = screen.width_in_pixels();

        /* XCB connection structure
         * An opaque structure that contain all data that XCB needs to communicate with an X
         * server. */

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
            &[
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (
                    xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
                ),
            ],
        );

        let surface = create_surface(
            &conn,
            &screen,
            window_id,
            i32::from(width),
            i32::from(height),
        )?;

        Ok((width, surface))
    }

    fn create_surface(
        conn: &xcb::Connection,
        screen: &xcb::Screen<'_>,
        id: u32,
        width: i32,
        height: i32,
    ) -> Result<cairo::XCBSurface> {

        let cairo_conn = unsafe {
            cairo::XCBConnection::from_raw_none(
                conn.get_raw_conn() as *mut cairo_sys::xcb_connection_t
            )
        };

        println!("cairo connected");
 
        let visual = unsafe {
            cairo::XCBVisualType::from_raw_none(
                &mut get_root_visual_type(conn, screen).base as *mut xcb::ffi::xcb_visualtype_t
                    as *mut cairo_sys::xcb_visualtype_t,
            )
        };

        println!("cairo visual created");

        let drawable = cairo::XCBDrawable(id);

        println!("cairo drawable created");

        let surface = cairo::XCBSurface::create(&cairo_conn, &drawable, &visual, width, height)
            .map_err(|status| anyhow!("XCBSurface::create: {}", status))?;

        Ok(surface)
    }

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

    #[derive(Debug)]
    pub enum BarError {}

    #[derive(Clone, Debug)]
    pub enum Position {
        Top,
        Bottom,
    }

    pub struct Bar {
        position: Position,
        pub conn: Rc<ewmh::Connection>,
        screen_idx: usize,
        window_id: u32,
        surface: cairo::XCBSurface,
        width: u16,
        height: u16,
        //contents: Vec
    }

    impl Bar {
        pub fn new(position: Position) -> Result<Bar> {
            println!("> new");

            /* XCB */
            let (xcb_conn, screen_idx) =
                xcb::Connection::connect(None).context("Failed to connect to X server")?;

            println!("xcb connected");

            let screen_idx = screen_idx as usize;

            /* EWMH */
            let ewmh_conn = ewmh::Connection::connect(xcb_conn)
                .map_err(|(e, _)| e)
                .context("Failed to wrap xcb::Connection in ewmh::Connection")?;

            println!("xcb wrapped in ewmh");

            let window_id = ewmh_conn.generate_id();
            println!("window_id: {}", window_id);

            /* Cairo */
            let height = 1;
            let (width, surface) = create_window(&ewmh_conn, screen_idx, window_id, height)?;
            println!("width: {}", width);

            // ---
            let bar = Bar {
                /* EWMH(XCB) */
                conn: Rc::new(ewmh_conn),
                window_id,
                screen_idx,

                /* Cairo */
                surface,

                /* Dimensions */
                width,
                height,

                /* Bar */
                position,
                //contents: Vec::new()
            };

            //&bar.redraw();

            bar.set_ewmh_properties();
            //xcb::map_window(&bar.conn, window_id); // not sure
            //bar.flush(); // flush again after ewmh stuff?

            Ok(bar)
        }

        /*
         * Get
         */

        fn screen(&self) -> Result<xcb::Screen<'_>> {
            println!("> screen");
            let screen = self
                .conn
                .get_setup()
                .roots()
                .nth(self.screen_idx)
                .ok_or_else(|| anyhow!("Invalid screen"))?;
            Ok(screen)
        }

        /*
         * EWMH
         */

        pub fn set_ewmh_properties(&self) {

            // https://specifications.freedesktop.org/wm-spec/wm-spec-latest.html#idm46035372564240

            println!("> set_ewmh_properties");
            println!("ewmh: WM_WINDOW_TYPE_DOCK");

            ewmh::set_wm_window_type(
                &self.conn,
                self.window_id,
                &[self.conn.WM_WINDOW_TYPE_DOCK()],
            );

            // TODO: Update _WM_STRUT_PARTIAL if the height/position of the bar changes?
            let mut strut_partial = ewmh::StrutPartial {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
                left_start_y: 0,
                left_end_y: 0,
                right_start_y: 0,
                right_end_y: 0,
                top_start_x: 0,
                top_end_x: 0,
                bottom_start_x: 0,
                bottom_end_x: 0,
            };

            match self.position {
                Position::Top => {
                    println!("ewmh: bar position: top");
                    strut_partial.top = u32::from(self.height)
                },
                Position::Bottom => {
                    println!("ewmh: bar position: bottom");
                    strut_partial.bottom = u32::from(self.height)
                },
            }

            ewmh::set_wm_strut_partial(&self.conn, self.window_id, strut_partial);
            println!("ewmh: set strut partials");

            //self.map_window();
            //self.flush();
        }

        fn recompute_dimensions(&mut self) -> Result<()> {
            //self.update_bar_height(height as u16)?;
            Ok(())
        }

        fn update_bar_height(&mut self, height: u16) -> Result<()> {
            println!("> update_bar_height");
            if self.height != height {
                self.height = height;

                // If we're at the bottom of the screen, we'll need to update the
                // position of the window.
                let y = match self.position {
                    Position::Top => 0,
                    Position::Bottom => self.screen()?.height_in_pixels() - self.height,
                };

                // Update the height/position of the XCB window and the height of the Cairo surface.
                let values = [
                    (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
                    (xcb::CONFIG_WINDOW_HEIGHT as u16, u32::from(self.height)),
                    (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
                ];

                xcb::configure_window(&self.conn, self.window_id, &values);

                self.map_window();

                self.surface
                    .set_size(i32::from(self.width), i32::from(self.height))
                    .unwrap();

                // Update EWMH properties - we might need to reserve more or less space.
                self.set_ewmh_properties();
                //self.redraw();
            }

            Ok(())
        }

        
        /*
         * Events
         */

        pub fn wait_for_event(&self) {
            println!("waiting for events");
            self.wait_for_event();
        }

        pub fn poll_for_event(&self) {
            println!("polling for events");
            self.conn.poll_for_event();
        }

        pub fn process_event(&mut self, event: xcb::GenericEvent) -> Result<()> {
            let expose = event.response_type() & !0x80 == xcb::EXPOSE;
            if expose {
                println!("Redrawing entire bar - expose event.");
                //self.redraw_entire_bar()?;
            }
            Ok(())
        }

        /*
         * Flush, Map
         */

        pub fn flush(&self) {
            self.conn.flush();
            println!("xcb connection flushed");
        }

        pub fn map_window(&self) {
            xcb::map_window(&self.conn, self.window_id);
            println!("xcb window mapped");
        }

        /*
         * Refresh
         */

        fn redraw_content(&mut self, idx: usize) -> Result<()> {
            /* ... */
            self.flush();
            Ok(())
        }

        pub fn redraw_bar(&mut self) -> Result<()> {
            println!("redraw");
            let height = 35;
            self.recompute_dimensions()?;
            //self.update_bar_height(height as u16)?;
            //self.flush();
            Ok(())
        }


        /*
         * Content
         */

        //pub fn add_content(&mut self, content: Vec<Text>) -> Result<usize> {
        //    //let idx = self.contents.len();
        //    /* ... */
        //    Ok(idx)
        //}

        //pub fn update_content(&mut self, idx: usize, content: Vec<Text>) -> Result<()> {
        //    Ok(())
        //}

        //pub fn update(&mut self) -> Result<()> {
        //    println!("update");
        //    self.redraw();
        //    Ok(())
        //}
    }
}
