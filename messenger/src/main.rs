use clap::{Command, Arg};
use messenger::server::Server;
use messenger::user::User;
use std::thread;
use std::time;
use std::io::stdin;
use std::sync::{Arc, Mutex};


fn main() {
    let app = Command::new("messenger")
        .about("Simple Messenger")
        .arg(Arg::new("mode").required(true))
        .arg(Arg::new("server").required(false))
        .get_matches();
    
        let mode = app.value_of("mode").unwrap();

        if mode == "server" {
            let mut handler = Server::new("127.0.0.1:9000");
            handler.listen();
            // loop {
            //     thread::sleep(time::Duration::from_secs(5));
            //     let mut clients = handler.clients.lock().unwrap();
            //     println!("{:?}",clients);
            
            //     for v in clients.iter_mut() {
            //         let val: Vec<u8> = v.read_stream();
            //         println!("read stream: {:?}",std::str::from_utf8(&val[..]));
            //         v.write_stream("Micheal here!");
            //     }
            // }
           // handler.broadcast_client();
            thread::sleep(time::Duration::from_secs(500));
        } else if mode == "client" {
            let mut user = User::new("127.0.0.1:9000").unwrap();
            let mut inputs = vec![];
            thread::spawn(|| {
                let mut line = String::new();
                print!("> ");
                let val = stdin().read_line(&mut line).unwrap();
                inputs.push(line);
            });
            loop {
                
              //  thread::sleep(time::Duration::from_secs(2));
                user.write_stream(&line);
              //  thread::sleep(time::Duration::from_secs(2));

            }

        } else {
            eprintln!("Usage: messenger <server / client>\nEg:\n\tmessenger server\n\tmessenger client 127.0.0.1:9000");
            return;
        }



}
