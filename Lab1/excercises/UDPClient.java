import java.io.IOException;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;

static final int port = 8008;
static final String message = "Message from Client";
static final int receiveLength = 100;

void send(DatagramSocket socket, InetAddress address) throws IOException {
    var buffer = message.getBytes();

    DatagramPacket toSend = new DatagramPacket(
        buffer,
        buffer.length,
        address,
        port
    );

    socket.send(toSend);

    System.out.format(
        "Sent \"%s\" to %s:%d\n", 
        message, 
        toSend.getAddress(),
        port
    );
}

void receive(DatagramSocket socket) throws IOException {
    var buffer = new byte[receiveLength];

    DatagramPacket toReceive = new DatagramPacket(
        buffer,
        buffer.length
    );

    socket.receive(toReceive);

    var message = new String(toReceive.getData());

    System.out.format(
        "Received \"%s\" from %s\n", 
        message, 
        toReceive.getAddress()
    );
}

void main() throws Exception {
    var address = InetAddress.getByName("localhost");

    try (
        DatagramSocket socket = new DatagramSocket()
    ) {
        send(socket, address);
        receive(socket);
    }
}
