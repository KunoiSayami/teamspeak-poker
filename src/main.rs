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


fn main() {
    let mut config = Ini::new();
    config.load("config.ini").unwrap();
    let port = config.getint("telnet", "port")
        .expect("Unable found option")
        .unwrap_or(25639);
    let auth_key = config.get("telnet", "key")
        .expect("Unable get authorized key");
    let mut telnet = Telnet::connect(("127.0.0.1", port as u16), 512)
        .expect("Couldn't connect to the server...");

    loop {
        let event = telnet.read().expect("Read error");

        match event {
            TelnetEvent::Data(buffer) => {
                let s = match String::from_utf8(Vec::from(buffer)) {
                    Ok(s) => s,
                    Err(_e) => {
                        eprintln!("Error while pasting utf8");
                        continue
                    }
                };
                print!("{}", s);
                if s.contains(r#"Use the "auth" command to authenticate yourself."#) {
                    telnet.write(format!("auth apikey={}", auth_key).as_bytes())
                        .expect("Read error");
                }
            },
            _ => {}
        }
    }
}