import WebSocket, { WebSocketServer } from 'ws';

const PORT = process.env.PORT ? parseInt(process.env.PORT) : 8080;
interface User {
    ws: WebSocket;
    id: string;
    nick: string;
    avatar: string;
    isAlive: boolean;
}

interface Message {
    messageType: string;
    data: string;
    dataArray: string[];
}

let users: User[] = [];

console.log(`Listening on port ${PORT}`);
const wss = new WebSocketServer({ port: PORT });

wss.on('connection', (ws: WebSocket) => {
    console.log('ws connected');

    ws.on('message', (data) => {
        const raw_data = data.toString();
        try {
            const parsed_data: Message = JSON.parse(raw_data);
            switch (parsed_data.messageType) {
                case 'register':
                    try {
                        const userData = JSON.parse(parsed_data.data);
                        const newUser: User = {
                            ws,
                            id: userData.id,
                            nick: userData.name,
                            avatar: userData.avatar,
                            isAlive: true,
                        };
                        users.push(newUser);
                        broadcast(JSON.stringify({
                            messageType: 'users',
                            data: JSON.stringify(
                                users.map((u) => ({
                                    id: u.id,
                                    name: u.nick,
                                    avatar: u.avatar,
                                }))
                            )
                        }));
                    } catch (err) {
                        console.error('Invalid register data', err);
                    }
                    break;

                case 'message':
                    const sender = users.find((u) => u.ws === ws);
                    if (sender) {
                        broadcast(JSON.stringify({
                            messageType: 'message',
                            data: JSON.stringify({
                                from: sender.id,
                                message: parsed_data.data,
                            }),
                        }));
                    }
                    break;
            }
        } catch (e) {
            console.log('Error in message', e);
        }
    });
});

const interval = setInterval(function ping() {
    const current_clients = Array.from(wss.clients);
    const updated_users = users.filter((u) => current_clients.includes(u.ws));
    if (updated_users.length !== users.length) {
        users = updated_users;
        broadcast(JSON.stringify({
            messageType: 'users',
            data: JSON.stringify(
                users.map((u) => ({
                    id: u.id,
                    name: u.nick,
                    avatar: u.avatar,
                }))
            )
        }));

    }
}, 5000);

const broadcast = (data: any) => {
    wss.clients.forEach((client) => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(data);
        }
    });
};
