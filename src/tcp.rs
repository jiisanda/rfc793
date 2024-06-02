use std::io;

enum State {
    Closed,
    Listen,
    // SyncRcvd,
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
    una: usize,         /// send acknowledge
    nxt: usize,         /// send next
    wnd: usize,         /// send window
    up: bool,           /// send urgent pointer
    wl1: usize,         /// segment sequence number used for last window update
    wl2: usize,         /// segment ack number used for last window update
    iss: usize,         /// initial send sequence number
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
    nxt: usize,         /// receive next
    wnd: usize,         /// receive window
    up: bool,           /// receive urgent pointer
    irs: usize,         /// initial receive sequence number
}

impl Default for State {
    fn default() -> Self {
        // State::Closed        // todo!
        Connection {
            state: State::Listen,
        }
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
                    tcph.destination_port(),
                    tcph.source_port(),
                    0,
                    0
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