use std::{collections::VecDeque, fs};

use aoc::{parse_memory, IntCode, Io, Result};

fn main() -> Result<()> {
    let mem = fs::read_to_string("./input/day23.in")?;
    let mem = parse_memory(&mem)?;
    let mut mems = (0..50).map(|_| mem.clone()).collect::<Vec<_>>();
    let mut peers = mems
        .iter_mut()
        .enumerate()
        .map(|(id, mem)| {
            let io = NetIo::new(id as i64);
            IntCode::new(io, mem)
        })
        .collect::<Vec<_>>();

    let mut nat_pack = [0; 2];
    let mut prev_y = -1;
    let mut idle_loops = 0;
    'outer: loop {
        let mut did_send = false;
        for i in 0..peers.len() {
            peers[i].io.recv_failed = false;
            for _ in 0..10 {
                peers[i].step()?;
                if let Some((addr, pack)) = peers[i].io.send_queue.pop_front() {
                    did_send = true;
                    if addr == 255 {
                        nat_pack = pack;
                    } else {
                        peers[addr as usize].io.enqueue(pack);
                    }
                }
            }
        }
        let idle = !did_send
            && nat_pack != [0; 2]
            && peers
                .iter()
                .all(|it| it.io.recv_queue.is_empty() && it.io.recv_failed);
        if idle {
            idle_loops += 1;
            if idle_loops == 2 {
                idle_loops = 0;
                if prev_y == nat_pack[1] {
                    eprintln!("prev_y = {:?}", prev_y);
                    break 'outer;
                }
                prev_y = nat_pack[1];
                peers[0].io.enqueue(nat_pack);
            }
        }
    }
    Ok(())
}
type Packet = [i64; 2];

struct NetIo {
    recv_failed: bool,
    recv_state: RecvState,
    recv_queue: VecDeque<Packet>,
    send_state: SendState,
    send_queue: VecDeque<(i64, Packet)>,
}

enum RecvState {
    Init(i64),
    Ready,
    Recv(i64),
}

enum SendState {
    Ready,
    Addr(i64),
    AddrSend(i64, i64),
}

impl NetIo {
    fn new(id: i64) -> NetIo {
        NetIo {
            recv_failed: false,
            recv_state: RecvState::Init(id),
            recv_queue: VecDeque::new(),
            send_state: SendState::Ready,
            send_queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, pack: Packet) {
        self.recv_failed = false;
        self.recv_queue.push_back(pack);
    }
}

impl Io for NetIo {
    fn read(&mut self) -> Result<i64> {
        let res = match self.recv_state {
            RecvState::Init(id) => {
                self.recv_state = RecvState::Ready;
                id
            }
            RecvState::Recv(byte) => {
                self.recv_state = RecvState::Ready;
                byte
            }
            RecvState::Ready => match self.recv_queue.pop_front() {
                None => {
                    self.recv_failed = true;
                    -1
                }
                Some([b1, b2]) => {
                    self.recv_state = RecvState::Recv(b2);
                    b1
                }
            },
        };

        Ok(res)
    }
    fn write(&mut self, value: i64) -> Result<()> {
        self.send_state = match self.send_state {
            SendState::Ready => SendState::Addr(value),
            SendState::Addr(addr) => SendState::AddrSend(addr, value),
            SendState::AddrSend(addr, b1) => {
                self.send_queue.push_back((addr, [b1, value]));
                SendState::Ready
            }
        };
        Ok(())
    }
}
