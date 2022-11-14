import { Server } from "@hapi/hapi";

function sleep(ms: number, value: string) {
  return new Promise((resolve) => setTimeout(() => resolve(value), ms));
}

export async function startServer(host: string, port: number): Promise<Server> {
  const server = new Server({ port, host });

  server.route({
    method: "GET",
    path: "/work",
    handler: async () => sleep(10 * 1000, `done something for 10 seconds\n`),
  });

  process.on("SIGTERM", async function () {
    console.log(`Received SIGTERM`);
    await server.stop({ timeout: 10 * 1000 });
    console.log(`Server stopped.`);
    process.exit(0);
  });
  await server.start();
  console.log(`Server running at: ${server.info.uri}, PID: ${process.pid}`);
  return server;
}

startServer("0.0.0.0", 3000);
