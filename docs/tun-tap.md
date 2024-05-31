# Tun-Tap Device Driver

Tun-tap provides packet reception and transmission. It can be seen as a simple Point-to-Point or Ethernet device. Rather
than receiving packets from physical media, a tun/tap device receives them from user space program and transmit them to 
user space program.
```text
+-----------------------------+
|         User Space          |
+-----------------------------+
|                     | TUN | |
|         Kernel      |_____| |
|                             |
+-----------------------------+
```

The TUN device going to treat anything that comes out of this as something that sort of came from the network. It's
going to treat this as a NIC a network interface card. Anytime the user program writes something that's going to go
through the TUN device, it's going to be treated as if it came from the network. This lets us emulate a network inside 
of user space. There a command, `ip addr` that lists all the network interfaces on the system. So anytime it tries to 
send a packet to the network interface, that's it going to appear out of the TUN device into the user space. And this 
allows us to implement TCP.

## Frame Format of TUN

- Flags: [2 bytes]
- Protocol: [2 bytes]
- Raw protocol (IP, IPv6, etc) frame.

## tun/tap carte for Rust

[link](https://docs.rs/tun-tap/latest/tun_tap/index.html), allows us to create a new interface;

```rust
iface = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun);
```

This gives us interface then we can call `tun_ap::Iface::recv()` to receive a message and `tun_ap::Iface::send()` to
send a message.
