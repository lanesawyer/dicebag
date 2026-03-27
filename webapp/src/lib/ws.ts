import { WebSocketServer } from 'ws';

const WS_PORT = 4322;

// Singleton — only start once per process
let wss: WebSocketServer | null = null;

function getServer(): WebSocketServer {
  if (!wss) {
    wss = new WebSocketServer({ port: WS_PORT });
  }
  return wss;
}

/** Broadcast a message to all connected clients. */
export function broadcast(message: string): void {
  const server = getServer();
  for (const client of server.clients) {
    if (client.readyState === 1 /* OPEN */) {
      client.send(message);
    }
  }
}
