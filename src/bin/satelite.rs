use std::vec;

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

struct GroundStation;

fn check_status(sat_id: &CubeSat) {
    println!("{:?}, {:?}", sat_id, StatusMessage::Ok);
}

fn fetch_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }

    fn send(self: &Self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(self: &mut Self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("Hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}, {:?}", sat, msg);
    }
}
