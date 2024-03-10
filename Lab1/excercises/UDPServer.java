import java.net.DatagramSocket;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.net.DatagramPacket;
import java.io.IOException;

static final int port = 8008;
static final String message = "Response from Server";
static final int receiveLength = 100;
static final String encoding = "cp1250";


DatagramPacket receive(DatagramSocket socket) throws IOException {
    var buffer = new byte[receiveLength];
    var toReceive = new DatagramPacket(buffer, buffer.length);

    socket.receive(toReceive);

    var message = new String(toReceive.getData(), encoding);
    var address = toReceive.getAddress();
    var port = toReceive.getPort();

    System.out.format(
        "Received \"%s\" from %s:%d\n", 
        message, 
        address,
        port
    );

    return toReceive;
}

void receiveAndReplyIncremented(DatagramSocket socket) throws IOException {
    var buffer = new byte[receiveLength];
    var toReceive = new DatagramPacket(buffer, buffer.length);

    socket.receive(toReceive);

    var value = ByteBuffer
        .wrap(buffer)
        .order(ByteOrder.LITTLE_ENDIAN)
        .getInt();
    
    var address = toReceive.getAddress();
    var port = toReceive.getPort();

    System.out.format(
        "Received \"%s\" from %s:%d\n", 
        value, 
        address,
        port
    );

    buffer = ByteBuffer
        .allocate(4)
        .order(ByteOrder.LITTLE_ENDIAN)
        .putInt(value + 1)
        .array();

    var toSend = new DatagramPacket(
        buffer, 
        buffer.length, 
        address, 
        port
    );

    socket.send(toSend);

    System.out.format(
        "Sent \"%s\" to %s:%d\n",
        value + 1,
        toSend.getAddress(),
        toSend.getPort()
    );
}


void reply(DatagramSocket socket, DatagramPacket request) throws IOException {
    var address = request.getAddress();
    var port = request.getPort();

    var buffer = message.getBytes();

    var toSend = new DatagramPacket(
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
        toSend.getPort()
    );
}

void main() throws Exception {
    try (
        DatagramSocket socket = new DatagramSocket(port)
    ) {
        // var request = receive(socket);
        // reply(socket, request);
        receiveAndReplyIncremented(socket);
    }
}
