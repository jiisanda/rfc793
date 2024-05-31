mod tcp;

use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;


#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16)
}

fn main() -> io::Result<()> {
    let mut connection: HashMap<Quad, tcp::State> = Default::default();
    let mut i_face = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
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
            Ok(iph) => {
                let src = iph.source_addr();
                let dest = iph.destination_addr();

                if iph.protocol() != etherparse::IpNumber(0x06) {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buffer[4+iph.slice().len()..n_bytes]) {
                    Ok(tcph) => {
                        let datai = 4 + iph.slice().len() + tcph.slice().len();
                        connection.entry(Quad {
                            src: (src, tcph.source_port()),
                            dest: (dest, tcph.destination_port()),
                        }).or_default().on_packet(&mut i_face, iph, tcph, &buffer[datai..n_bytes])?;
                    }
                    Err(e) => {
                        eprintln!("ignoring wired tcp packet {:?}", e)
                    }
                }
            },
            Err(e) => {
                eprintln!("ignoring wired packet {:?}", e)
            }
        }
    }
    // Ok(())
}
