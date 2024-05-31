use std::io;

pub(crate) enum State {
    Closed,
    Listen,
    // SyncRcvd,
    // Estab,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed        // todo!
        State::Listen
    }
}


// here <'a> is the lifetime of the packet itself
impl State  {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize>{
        let mut buff = [0u8; 1500];
        // let mut unwritten = &mut buff[..];
        match *self{
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // only syn packet is expected
                    return Ok(0);
                }

                // need to start establishing connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(), tcph.source_port(), 0, 0
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                // wrapping it into IP packet, as we are sending it back to them
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len() as u16,
                    64,
                    etherparse::IpNumber::TCP,
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );

                // headers
                let unwritten = {
                    let mut unwritten = &mut buff[..];
                    ip.unwrap().write(&mut unwritten).expect("TODO: panic message");
                    syn_ack.write(&mut unwritten).expect("TODO: panic message");
                    unwritten.len()
                };
                nic.send(&buff[..unwritten])
            }
        }
    }
}