use std::io;

use tun_tap::Mode;

fn main() -> io::Result<()> {
    let i_face = tun_tap::Iface::new("tun0", Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    loop {
        let n_bytes = i_face.recv(&mut buffer[..])?;

        // TUN Frame Format: flags: 2 bytes; proto: 2 bytes;
        let _eth_flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        if eth_proto != 0x0800 {
            continue;           // no ipv4
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4..n_bytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dest = p.destination_addr();
                let proto = p.protocol();
                eprintln!(
                    "{} → {} {:?}b of protocol {:}",
                    src,
                    dest,
                    p.payload_len(),
                    proto
                );
            },
            Err(e) => {
                eprintln!("ignoring wired packet {:?}", e)
            }
        }
    }
    // Ok(())
}
