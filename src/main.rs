use std::io;

use tun_tap::Mode;

fn main() -> io::Result<()> {
    let i_face = tun_tap::Iface::new("my_tun", Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    let nbytes = i_face.recv(&mut buffer[..])?;
    eprintln!("read {} bytes: {:x?}", nbytes, &buffer[..nbytes]);
    Ok(())
}
