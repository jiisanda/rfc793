use std::io;

use tun_tap::Mode;

fn main() -> io::Result<()> {
    let i_face = tun_tap::Iface::new("tun0", Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    loop {
        let n_bytes = i_face.recv(&mut buffer[..])?;

        // TUN Frame Format: flags: 2 bytes; proto: 2 bytes;
        let flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        if proto != 0x0800 {
            continue;           // no ipv4
        }

        match etherparse::Ipv4Slice::from_slice(&buffer[4..n_bytes]) {
            Ok(p) => {
                eprintln!(
                    "read {} bytes (flags: {:x}, proto: {:x}): {:x?}",
                    n_bytes - 4,
                    flags,
                    proto,
                    p
                );
            },
            Err(e) => {
                eprintln!("ignoring wired packet {:?}", e)
            }
        }
    }
    // Ok(())
}
