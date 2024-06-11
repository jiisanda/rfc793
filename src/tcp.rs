use std::io;

enum State {
    Closed,
    Listen,
    SynRcvd,
    // Estab,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    rcv: ReceiveSequenceSpace,
}

/// State of Send Sequence Space (RFC 793-S3.2 F4)
/// ```
///            1         2          3          4
///       ----------|----------|----------|----------
///              SND.UNA    SND.NXT    SND.UNA
///                                   +SND.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers of unacknowledged data
/// 3 - sequence numbers allowed for new data transmission
/// 4 - future sequence numbers which are not yet allowed
/// ```
struct SendSequenceSpace {
    una: u32,         // send acknowledge
    nxt: u32,         // send next
    wnd: u16,         // send window
    up: bool,           // send urgent pointer
    wl1: usize,         // segment sequence number used for last window update
    wl2: usize,         // segment ack number used for last window update
    iss: u32,         // initial send sequence number
}

/// State of Receive Sequence Space (RFC 793-S3.2 F5)
/// ```
///                       1          2          3
///                   ----------|----------|----------
///                           RCV.NXT    RCV.NXT
///                                     +RCV.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers allowed for new reception
/// 3 - future sequence numbers which are not yet allowed
/// ```
struct ReceiveSequenceSpace {
    nxt: u32,         // receive next
    wnd: u16,         // receive window
    up: bool,           // receive urgent pointer
    irs: u32,         // initial receive sequence number
}


// here <'a> is the lifetime of the packet itself
impl Connection  {
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buff = [0u8; 1500];
        // let mut unwritten = &mut buff[..];
        if !tcph.syn() {
            // only syn packet is expected
            return Ok(None);
        }

        // new state for new connection
        let iss = 0;
        let mut c = Connection {
            state: State::SynRcvd,
            send: SendSequenceSpace {
                // decide on things to send
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            rcv: ReceiveSequenceSpace {
                // keep track of sender info
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
                irs: tcph.sequence_number(),
            }
        };

        // need to start establishing connection
        let mut syn_ack = etherparse::TcpHeader::new(
            tcph.destination_port(),
            tcph.source_port(),
            c.send.iss,
            c.send.wnd,
        );
        syn_ack.acknowledgment_number = c.rcv.nxt;       // we are expecting the next syn
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

        eprintln!("got ip headers: \n {:02x?}", iph);
        eprintln!("got tcp header: \n {:02x?}", tcph);

        // headers
        let unwritten = {
            let mut unwritten = &mut buff[..];
            ip.unwrap().write(&mut unwritten).expect("TODO: panic message");
            syn_ack.write(&mut unwritten).expect("TODO: panic message");
            unwritten.len()
        };

        eprintln!("responding with {:0x?}", &buff[..buff.len() - unwritten]);

       nic.send(&buff[..unwritten])?;
        Ok(Some(c))
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {

        unimplemented!();
    }
}
