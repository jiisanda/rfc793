pub(crate) enum State {
    Closed,
    Listen
}

impl Default for State {
    fn default() -> Self {
        State::Closed        // todo!
    }
}


// here <'a> is the lifetime of the packet itself
impl State  {
    pub fn on_packet<'a>(
        &mut self,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} → {}:{} {:?}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        )
    }
}