#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str, data: Vec<u8>) -> File {
        File {
            name: String::from(name),
            data,
            state: FileState::Closed,
        }
    }

    fn open(&mut self) -> Result<bool, String> {
        self.state = FileState::Open;
        Ok(true)
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, String> {
        if self.state == FileState::Closed {
            return Err(String::from("Cannot raed from the file as file is closed"));
        }
        let mut data = self.data.clone();
        let read_length = data.len();
        buffer.append(&mut data);
        Ok(read_length)
    }

    fn close(&mut self) -> Result<bool, String> {
        self.state = FileState::Closed;
        Ok(true)
    }
}

fn main() {
    let data = vec![12, 33, 44, 55, 22, 33, 44];
    let mut f = File::new("hello.txt", data);

    let mut buffer: Vec<u8> = vec![];

    f.open().unwrap();
    let read_length = f.read(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    f.close().unwrap();
    println!("{:?}", f);
    println!("{} data read", read_length);
    println!("{}", text);
}
