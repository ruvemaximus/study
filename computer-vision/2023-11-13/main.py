import socket
from math import sqrt
from typing import List

import numpy as np
from skimage.measure import label, regionprops


HOST = "84.237.21.36"
PORT = 5152
DATA_SIZE = 40_002


def recvall(sock, n) -> List[int]:
    data = bytearray()
    while len(data) < n:
        packet = sock.recv(n - len(data))
        if not packet:
            return None
        data.extend(packet)
    return data


with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.connect((HOST, PORT))
    for _ in range(10):
        sock.send(b"get")

        _bytes = recvall(sock, DATA_SIZE)
        shape = _bytes[0:2]

        image = np.frombuffer(_bytes[2:DATA_SIZE], dtype="uint8").reshape(*shape)
        image[image > 0] = 1

        regs = regionprops(label(image))
        centers = [reg.centroid for reg in regs]

        dist = round(
            sqrt(
                (centers[0][0] - centers[1][0]) ** 2
                + (centers[0][1] - centers[1][1]) ** 2
            ), 1
         )

        print(f'{dist=}')
        sock.send(str(dist).encode())
        response = sock.recv(4)
        print(f'{response=}')
