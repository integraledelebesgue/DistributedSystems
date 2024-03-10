import socket as sockets
from socket import socket as Socket

ADDRESS = '127.0.0.1'
RECEIVE = 9009
SEND = 8008
RECEIVE_LENGTH = 100
MESSAGE = 'Żółta gęś'
NUMBER = 300
ENCODING = 'cp1250'


def as_bytes(message: str) -> bytes:
    return bytes(message, ENCODING)


def bytify(number: int) -> bytes:
    return number.to_bytes(4, 'little')


def as_str(buffer: bytes) -> str:
    return str(buffer, ENCODING)


def as_int(buffer: bytes) -> int:
    return int.from_bytes(buffer, 'little')


def send(socket: Socket) -> None:
    buffer = as_bytes(MESSAGE)

    socket.sendto(
        buffer,
        (ADDRESS, SEND)
    )

    message = as_str(buffer)

    print(f'Sent "{message}" to {ADDRESS}:{SEND}')


def send_number(socket: Socket) -> None:
    buffer = bytify(NUMBER)

    socket.sendto(
        buffer,
        (ADDRESS, SEND)
    )

    message = as_int(buffer)

    print(f'Sent "{message}" to {ADDRESS}:{SEND}')


def receive(socket: Socket) -> None:
    buffer, address = socket.recvfrom(RECEIVE_LENGTH)
    message = as_str(buffer)

    print(f'Received "{message}" from {address}')


def receive_number(socket: Socket) -> None:
    buffer, address = socket.recvfrom(RECEIVE_LENGTH)
    message = as_int(buffer)

    print(f'Received "{message}" from {address}')


if __name__ == '__main__':
    socket = Socket(sockets.AF_INET, sockets.SOCK_DGRAM)
    socket.bind(('', RECEIVE))

    # send(socket)
    # receive(socket)

    send_number(socket)
    receive_number(socket)
