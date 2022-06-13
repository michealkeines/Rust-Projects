use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::str;
use std::time;

pub struct User {
    pub stream: Arc<Mutex<TcpStream>>
}

fn convert_to_vector(message: &str) -> Vec<u8> {
    if message.len() == 250 {
        message.as_bytes().to_vec()
    } else if message.len() < 250 {
        let mut temp: Vec<u8> = vec![0;250];
        let mut count = 0;
        for c in message.as_bytes() {
            temp[count] = *c;
            count += 1;
        }
       // println!("updaed");
        temp
    } else {
        let mut temp: Vec<u8> = vec![0;250];
        let mut count = 0;
        for c in message.as_bytes() {
            if count == 249 {
                return temp;
            }
            temp[count] = *c;
            count += 1;
        }
        temp
    }
}

fn strip_tail(message: Vec<u8>) -> Vec<u8> {
    let mut end: usize = 249;
 // println!("vector: {:?}", message);
    while message[end] == 0 {
        if end == 0 && message[end] == 0 {
            end = 249;
            break;
        }
        end -= 1;
    }
    let mut temp = vec![0;end];
    let mut count = 0;
    for c in message {
        if count == end{
            break;
        }
        temp[count] = c;
        count += 1;
    }
    if temp[0] == 0 {
        let t = "";
        t.as_bytes().to_vec()
    } else {
        temp
    }
    
}

impl User {
    pub fn new(ip_port: &str) -> Option<Self> {
        let stream = TcpStream::connect(&ip_port);
        match stream {
            Ok(s) => {
                Some(User {
                    stream: Arc::new(Mutex::new(s))
                })
            }
            _ => {
                None
            }
        }
    }

    pub fn write_stream(&mut self, message: &str) {
        let msg = convert_to_vector(message);
        let mut lock = self.stream.lock().unwrap();
        lock.write(&msg);
       // println!("written to stream");
    }

    pub fn read_stream(&mut self) -> Vec<u8> {
        let mut temp: Vec<u8> = vec![0;250];
        let mut lock = self.stream.lock().unwrap();
        lock.set_read_timeout(Some(time::Duration::from_secs(5)));
        lock.read_exact(&mut temp[..]);
        let temp = strip_tail(temp);
        return temp;
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_convert() {
        let s1: &str = "asfafasdfhsfjl";
        let v = convert_to_vector(s1);
        println!("{:?}",v);
        println!("{}",v.len());
        let v = strip_tail(v);
        let s = str::from_utf8(&v);
        println!("{:?}",s);
    }

}