pub(crate) struct State {

}

impl Default for State {
    fn default() -> Self {
        State {}        // todo!
    }
}

impl State  {
    pub fn on_packet(&mut self, iph: etherparse::Ipv4HeaderSlice, tcph: etherparse::TcpHeaderSlice, data: &[u8]) {
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