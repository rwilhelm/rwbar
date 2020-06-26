use xcb;

pub fn xcb_connect() -> Result<(Rc<ewmh::Connection>, impl Stream<Item = ()>)> {

    let (conn, screen_num) = xcb::Connection::connect(None).context("bla");



    Ok((conn, stream))
}

fn main() {

    println!("Connecting to X server");


}
