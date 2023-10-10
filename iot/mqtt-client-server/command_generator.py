import serial
import time

respones = {'d': 7, 'u': 6}
port = "COM4"
connection = serial.Serial(port, timeout=1)


def send_command(cmd: str, response_len: int) -> str:
    connection.write(cmd.encode())
    if response_len != 0:
        resp = connection.read(response_len)
        str_resp = resp.decode()
    return str_resp


while True:
    for command in respones:
        resp = send_command(command, respones[command])
        print(resp)
        time.sleep(1)
