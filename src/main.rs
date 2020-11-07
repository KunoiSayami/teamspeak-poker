/*
 ** Copyright (C) 2020 KunoiSayami
 **
 ** This file is part of teamspeak-poker and is released under
 ** the AGPL v3 License: https://www.gnu.org/licenses/agpl-3.0.txt
 **
 ** This program is free software: you can redistribute it and/or modify
 ** it under the terms of the GNU Affero General Public License as published by
 ** the Free Software Foundation, either version 3 of the License, or
 ** any later version.
 **
 ** This program is distributed in the hope that it will be useful,
 ** but WITHOUT ANY WARRANTY; without even the implied warranty of
 ** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 ** GNU Affero General Public License for more details.
 **
 ** You should have received a copy of the GNU Affero General Public License
 ** along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use telnet::{Telnet, TelnetEvent};
use configparser::ini::Ini;
use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};

fn get_value<'a>(option: &'a [&str], index: usize) -> &'a str {
    option.get(index).unwrap().split('=').last().unwrap()
}

#[derive(Debug)]
struct Client {
    clid: i32,
    cid: i32,
    client_database_id: i32,
    client_nickname: String,
    client_type: i32
}

impl Client {
    fn new(s: &str) -> Client {
        let options: Vec<&str> = s.split_whitespace().collect();
        if options.len() != 5 {
            panic!("")
        }
        Client {
            clid: get_value(&options, 0).parse().unwrap(),
            cid: get_value(&options, 1).parse().unwrap(),
            client_database_id: get_value(&options, 2).parse().unwrap(),
            client_nickname: get_value(&options, 3).parse().unwrap(),
            client_type: get_value(&options, 4).parse().unwrap()
        }
    }

    /*fn print(&self) {
        println!("client id: {}, nickname: {}", self.clid, self.client_nickname)
    }*/

    fn to_string(&self) -> String {
        format!("client id: {}, nickname: {}", self.clid, self.client_nickname)
    }
}

#[derive(Debug)]
struct Clients {
    items: Vec<Client>
}

impl Clients {
    fn new(s: String) -> Clients {
        let mut c = Clients{ items: vec![] };
        let mut input = s.clone();
        if input.contains("error id=0 msg=ok") {
            input = String::from(s.split('\n').next().unwrap());
        }
        for client_str in input.split('|') {
            c.items.push(Client::new(client_str));
        };
        c
    }

    /*fn print(&self) {
        for client in &self.items {
            client.print()
        }
    }*/

    fn to_string(&self) -> String {
        (&self.items).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")
    }
}


fn main() {
    let mut config = Ini::new();
    config.load("config.ini").unwrap();
    let port = config.getint("telnet", "port")
        .expect("Unable found option")
        .unwrap_or(25639);
    let api_key = config.get("telnet", "key")
        .expect("Unable get authorized key");
    let mut telnet = Telnet::connect(("127.0.0.1", port as u16), 512)
        .expect("Couldn't connect to the server...");

    let mut login = false;
    let mut confirm_login = false;
    let mut select_client = false;
    let mut requested_list = false;
    let clients= loop {
        let event = telnet.read().expect("Read error");

        if let TelnetEvent::Data(buffer) = event {
            let s = match String::from_utf8(Vec::from(buffer)) {
                Ok(s) => s,
                Err(_e) => {
                    eprintln!("Error while pasting utf8");
                    continue
                }
            };

            if !login && s.contains(r#"Use the "auth" command to authenticate yourself."#) {
                telnet.write(format!("auth apikey={}\n\r", api_key).as_bytes())
                    .expect("Read error");
                login = true;
            } else if !confirm_login && s.contains("error id=0 msg=ok") {
                telnet.write("use\n\r".as_bytes())
                    .expect("Read error");
                confirm_login = true;
            } else if !select_client && s.contains("selected schandlerid=1") {
                select_client = true;
            }
            if select_client {
                if !requested_list {
                    telnet.write("clientlist\n\r".as_bytes())
                        .expect("Read error");
                    requested_list = true;
                } else if s.contains("client_database_id") {
                    break Clients::new(s);
                }
            }
        }
    };

    print!("Client list:\n {}\n Please input which client you want to poke: ", clients.to_string());
    stdout().flush().unwrap();
    let clid ='_outside_loop: loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .unwrap();
        let clid: i32 = match input.trim().parse::<i32>() {
            Ok(id) => id,
            Err(e) => {
                print!("Please input a valid number, {:?}: ", e);
                stdout().flush().unwrap();
                continue
            }
        };
        if clid <= 0 {
            println!("Exited");
            return
        }
        for client in &clients.items {
            if client.clid == clid {
                break '_outside_loop client.clid;
            }
        }
        print!("Id not found, please try again: ");
        stdout().flush().unwrap();
    };

    println!("Set clid to {}", clid);
    for _times in 0..15 {
        telnet.write(format!("clientpoke msg= clid={}\n\r", clid).as_bytes())
            .expect("Write error");
        thread::sleep(Duration::from_millis(1100));
    }
}