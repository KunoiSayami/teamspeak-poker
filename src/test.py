# -*- coding: utf-8 -*-
# test.py
# Copyright (C) 2017-2020 KunoiSayami
#
# This module is part of teamspeak-poker and is released under
# the AGPL v3 License: https://www.gnu.org/licenses/agpl-3.0.txt
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
import socket
from configparser import ConfigParser


def main():
    config = ConfigParser()
    config.read('config.ini')
    port = config.getint("telnet", "port", fallback=25639)
    api_key = config.get("telnet", "key")
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(('127.0.0.1', port))
    while True:
        data = s.recv(512)
        if 'auth" command' in data.decode():
            s.send(f'auth apikey={api_key}\n\r'.encode("ascii"))
        print(data)


if __name__ == '__main__':
    main()