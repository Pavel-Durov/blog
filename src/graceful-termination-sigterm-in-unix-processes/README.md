## Graceful Termination in K8S, SIGTERM and UNIX processes

(Or how to write zero downtime horizontally scalable applications in the cloud.)

## Abstract
In this article, we're going to cover `UNIX` process signals and `SIGTERM` signals in particular. We will cover how to handle them with practical examples using [Node](https://nodejs.org/en/)[1], [TypeScript](https://www.typescriptlang.org/) [2], [Docker](https://www.docker.com/)[3] and [Kind](https://kind.sigs.k8s.io/)[4] local cluster.

## UNIX Signals and SIGTERM

Unix-based operating systems (OS) have multiple processes. OS uses software [interrupts](https://en.wikipedia.org/wiki/Interrupt)[5] (aka signals) as a way to communicate with the running processes, these signals are indicating that some sort of event has occurred and they can vary in their intent and purpose.

Here's a list of signals available on my machine:
```shell
$ kill -l
 1) SIGHUP       2) SIGINT       3) SIGQUIT      4) SIGILL       5) SIGTRAP
 6) SIGABRT      7) SIGEMT       8) SIGFPE       9) SIGKILL     10) SIGBUS
11) SIGSEGV     12) SIGSYS      13) SIGPIPE     14) SIGALRM     15) SIGTERM
16) SIGURG      17) SIGSTOP     18) SIGTSTP     19) SIGCONT     20) SIGCHLD
21) SIGTTIN     22) SIGTTOU     23) SIGIO       24) SIGXCPU     25) SIGXFSZ
26) SIGVTALRM   27) SIGPROF     28) SIGWINCH    29) SIGINFO     30) SIGUSR1
31) SIGUSR2
```
There're a few, but we're going to focus on `SIGTERM`.

`SIGTERM` signal is a way for the operating system to terminate a program gracefully. By graceful, we mean that the program is given time to perform the final cleanup before shutdown. Depending on the application, cleanup tasks can vary. Interestingly enough, Unix processes can block and ignore `SIGTERM`. But if we want to have quality process/service we need to handle these signals as they intended, with respect, otherwise our process will be shutting down with force.

## Unix Process as HTTP Server
For demonstration, we're going to create a sample HTTP server using [TypeScript](https://www.typescriptlang.org/)[1], and [hapi](https://hapi.dev/)[2].

Let's create [NPM](https://www.npmjs.com/)[6] project and follow the prompt:
```
$ npm init
...

```
Install dependencies:
```
$ npm install @hapi/hapi typesript @types/node @types/node @types/hapi__hapi 
```
We're not going to bother with the split of development and production dependencies here 😉.

Create a local file, call it `index.ts` (or whatever you feel like today, go wild):
```TypeScript
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

  await server.start();
  console.log(`Server running at: ${server.info.uri}, PID: ${process.pid}`);
  return server;
}

startServer("0.0.0.0", 3000);

```
Here, we start the local server on `0.0.0.0` host and `3000` port. We also configured a single endpoint `GET /work` that simulates something that takes time to compute - 10 seconds in our case. In a real-case scenario that might be the time needed to perform some kind of a Database query.

> I like node, its so cool that in 27 lines of code you can define a server, you actually need less, cause I added sleep function etc. But yeah, its great 😎.

Let's run our server:

```shell
$ ./node_modules/.bin/ts-node ./index.ts
Server running at: http://0.0.0.0:3000, PID: 16544
```
All good, we have server running!

Now let's send `GET /work` HTTP request to our endpoint. Pick any networking tool of your choice, I'm going to use [CURL](https://curl.se/) [7]:

```shell
$ curl http://0.0.0.0:3000/work
done something for 10 seconds
```

So far so good. But what will happen to that HTTP request if the server is suddenly killed, terminated or in other words, is no more 🤔. What response would we get on the client side? Will we get anything at all? What will be serving the request after the server is dead? Many questions! Let's give it a go:

Send another request to the server:
```shell
$ curl http://0.0.0.0:3000/work
done something for 10 seconds
```
But this time, while the server is doing the simulated work for 10 seconds - let's KILL it 😈!

```
$ kill -15 19346 
```
> Note: I'm using Process Id (PID) I got from index.ts output! Your local PID would we something else for sure! If it not, then its destiny and definitely send me an email 🤓.

One might wonder, why not just terminate the shell process of the server by pressing the `CTRL`+`C` keys? That would send `SIGINT` signal and we want `SIGTERM`!

Ok, so what happened to our client connection when the server is shut down? That's what happens:
```shell
$ curl http://0.0.0.0:3000/work
curl: (52) Empty reply from server
```
It got an empty reply, meaning that it got no information, nada. Let's add `-v` (stands for verbose) flag to our CURL command to see more information.

```
λ curl -v http://0.0.0.0:3000/work
*   Trying 0.0.0.0:3000...
* Connected to 0.0.0.0 (127.0.0.1) port 3000 (#0)
> GET /work HTTP/1.1
> Host: 0.0.0.0:3000
> User-Agent: curl/7.84.0
> Accept: */*
> 
* Empty reply from server
* Closing connection 0
curl: (52) Empty reply from server
```

Looks like the server just closed the connection abruptly 👀. That's not good. What if this happens to the server clients in production? If you're running your application in one of the cloud orchestration tools, containers being shut down might be not out of ordinary. 

I am using [Kubernetes (K8S)](https://kubernetes.io/)[8] so I'm going to talk about that. K8S is like OS for the cloud. K8S might terminate the running [Pods](https://kubernetes.io/docs/concepts/workloads/pods/)[9] (aka containers) at will, after all, the whole purpose of K8S is to orchestrate the distributed system. If one of the Pods is requesting too many resources or if the application is being scaled down, the container might get a signal from K8S that it's time to go 😢. 
That's why we have the notion of signals, to gracefully terminate our containers ☝️.


## Service Graceful Termination
In the previous section, we talk about how our processes or containers might receive different signals.
When K8S needs to terminate a Pod, it will send `SIGTERM`. That way our service is not just being cut out of resources, instead it will have some time to perform finalisation tasks.

Let's implement this logic in our node server:

```TypeScript
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

```

We added a listener on `SIGTERM` event. When such events occur, we're stopping the server but we're not just terminating it. Between the time that `SIGTERM` arrives and the specified `timeout` parameter, our server will refuse to accept any new requests and will finalise ongoing requests.

We can test that statement! Start the server again:
```
$ ts-node ./index.ts
Server running at: http://0.0.0.0:3000, PID: 67116
```

Run CURL request and terminate the server:
```
$ curl -v http://0.0.0.0:3000/work
```
We should see that the request is finished and a response sent back to the client - no more `(52) Empty reply from server` errors.
If we try to access the server in the time frame between `SIGTERM` and the actual shutdown we will get: 

```
$ curl -v http://0.0.0.0:3000/work
*   Trying 0.0.0.0:3000...
* connect to 0.0.0.0 port 3000 failed: Connection refused
* Failed to connect to 0.0.0.0 port 3000 after 3 ms: Connection refused
* Closing connection 0
curl: (7) Failed to connect to 0.0.0.0 port 3000 after 3 ms: Connection refused
```

The same error that we'll get if there's no server at all!

But that's all you need to do gracefully termination of node processes. Pretty easy 😃.

# Sending SIGTERM to K8S Pds

As we mentioned, whenever the K8S Pod is terminated, it is sent a `SIGTERM` signal.
Similarly to the way we used `kill` command for local processes, we can terminate pods using `delete` command:
```shell
$ kubectl delete pod my-pod-qgldf
```
When K8S decides for whatever reason to terminate the Pod, the `SIGTERM` signal will be sent to it, then to the Docker container, and eventually to the running process.
You don't have to believe me, give it a go.

## K8S sample application

We're going to reuse the server code, but containarise it and deploy it on the local K8S cluster.

Our Dockerfile:
```Dockerfile
FROM node:19-bullseye
WORKDIR /app
COPY index.ts package.json package-lock.json /app/
RUN npm install
EXPOSE 3000
ENTRYPOINT ["/app/node_modules/.bin/ts-node", "index.ts"]
```

Build docker container:
```shell
$ docker build -t poc -f ./Dockerfile .
```
First let's run it and see if we can get `SIGTERM` related logs:
```shell
$ docker run -t poc:latest
```
Get the docker id:
```shell
$ docker ps
CONTAINER ID   IMAGE                  
86b0a46730ba   poc:latest             
```
And stop it:
```shell
$ docker stop 86b0a46730ba
```
We should see the docker run command terminated as well as our docker container with the same output as we had with the process.
```
$  docker run -t poc:latest
Server running at: http://0.0.0.0:3000, PID: 1
Received SIGTERM
Server stopped.
```

Now let's do some K8S!

First, we need to load our container image to the cluster:
```shell
$ kind load docker-image poc:latest --name my-cluster
```

K8S deployment manifest:
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: poc-namespace
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: poc-deployment
  namespace: poc-namespace
spec:
  selector:
      matchLabels:
        app: poc-app
  template:
    metadata:
      labels:
        app: poc-app
    spec:
      containers:
      - name: poc
        image: poc:latest
        ports:
        - containerPort: 3000
```

Deploy to our cluster:
```yaml
$ kubectl apply -f ./deployment.yaml 
namespace/poc-namespace created
deployment.apps/poc-deployment created
```
If you stream pod logs:
```bash
$ k logs -f poc-deployment-bf749f576-wfmv9
```
And then delete the pod:
```
$ kubectl delete pod poc-deployment-bf749f576-wfmv9
```
Should get the same results:
```shell
$ k logs -f poc-deployment-bf749f576-wfmv9
Server running at: http://0.0.0.0:3000, PID: 1
Received SIGTERM
Server stopped.
```
And that's a wrap 🌯!

# Summary
In this article, we covered `SIGTERM` signals, how they are used in Unix-based and cloud-based systems as well as showed practical applications of handling to handle these signals and their potential impact of it.

`SIGTERM` handling is essential for [horizontally scalable](https://en.wikipedia.org/wiki/Scalability#Horizontal_(scale_out)_and_vertical_scaling_(scale_up))[10] cloud applications. It allows applications to scale up and down on demand without impacting the client's stability. 

# References

[1] https://nodejs.org/en/

[2] https://www.typescriptlang.org/

[3] https://www.docker.com/

[4] https://kind.sigs.k8s.io/

[5] https://en.wikipedia.org/wiki/Interrupt

[6] https://www.npmjs.com/

[7] https://curl.se/

[8] https://kubernetes.io/

[9] https://kubernetes.io/docs/concepts/workloads/pods/

[10] https://en.wikipedia.org/wiki/Scalability#Horizontal_(scale_out)_and_vertical_scaling_(scale_up)