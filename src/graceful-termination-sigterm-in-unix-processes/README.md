## Graceful Termination - SIGTERM in UNIX processes, containers and K8S

## Abstract

In this article we're going to talk about `UNIX` process signals and `SIGTERM` signal specifically. We will cover how to handle them with practical examples using [Node](https://nodejs.org/en/)[1], [TypeScript](https://www.typescriptlang.org/) [2], [Docker](https://www.docker.com/)[3] and [Kind](https://kind.sigs.k8s.io/)[4] local cluster.

## SIGTERM Signal

The operating system (OS) you're working probably had multiple processes running in the background (unless you're on some kind of exotic machine). These processes are managed by the OS. Signals is one way for the OS to interact with the running process, these signals can are software interrupts indicating that some sort of event has occurred. These signals can vary in their intent and purpose.

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
There're a few, but in this article we're going to focus on `SIGTERM`.

`SIGTERM` signal is a way for the operating system to terminate a program in a graceful way. By graceful we mean that the program is given time to perform final cleanup before its shutdown. Depending on application cleanup tasks can vary. Interestingly enough, Unix processes can block and ignore SIGTERM. But if you want to play nice wit the OS you should probably respect the signals, otherwise you'll end up shutting down using force.

## Unix Process as HTTP Server

For the purpose of demonstration, we're going to create a sample HTTP server using [TypeScript](https://www.typescriptlang.org/) [1] , and [hapi](https://hapi.dev/) [2] server for that task.

Create [NPM](https://www.npmjs.com/) [5] project and follow the prompt:
```
$ npm init
```
Install dependencies:
```
$ npm install @hapi/hapi typesript @types/node @types/node @types/hapi__hapi 
```
We're not going to bother with the split of development and production dependencies here 😉.

Create local file, call it `index.ts` (or whatever you feel like today, go wild):

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
So do we have here? We start the local server on `0.0.0.0` host and `3000` port. We also configured single endpoint `GET /work` that just simulates something that takes time to compute - 10 seconds in our case.

>I like node, its so cool that in 27 lines of code you can define a server. Well, you actually need less, cause I added sleep function etc. But yeah, its great 😎.

Let's run our server:

```shell
$ ./node_modules/.bin/ts-node ./index.ts
Server running at: http://0.0.0.0:3000, PID: 16544
```

Send http request to our endpoint, and pick any networking tool of your choice, I'm going to use [CURL](https://curl.se/) [6] cause its minimal and overall cool!

```shell
$ curl http://0.0.0.0:3000/work
done something for 10 seconds
```
So far so good. But what will happen to that HTTP request if the server is suddenly killed, terminated or is other words is no more 🤔. What response would we get on the client side? Will we get anything at all? What will be serving the request after the server is dead? Many questions! Let's just give it a go:

Send another request to the server:
```shell
$ curl http://0.0.0.0:3000/work
done something for 10 seconds
```
But this time,  while the server is doing something for a duration of 10 seconds - let's KILL it!

```
$ kill -15 19346 
```
> Note: I'm using Process Id (PID) I got from index.ts output! Your local PID would we something else for sure! If it not please send me an email 🤓.

One might wonder, whey not kist terminate the shell process of the server by pressing `CTRL`+`C` keys? That would send `SIGINT` and we're focusing on `SIGTERM` as we said!

Ok, so what happened to our client connection when the server is shutdown?

```shell
$ curl http://0.0.0.0:3000/work
curl: (52) Empty reply from server
```
It got empty reply, meaning that it got no information. Let's add `-v` (stands for verbose) flag to our curl command to see more ifo.

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

Looks like the server just closed the connection abruptly 👀. That's not good. If you're running your application in the cloud with one of the orchestration tools that event of getting your container shutdown might be not out of ordinary. I am using [Kubernetes (K8S)](https://kubernetes.io/)[7] so I'm going to talk about that. K8S is like OS but for the cloud, same idea. K8S might terminate the running Pods (aka containers) at will, after all the whole purpose of K8S is to orchestrate distributed system. If one of the Pods are requesting too many resources or if application is being scaled-down, the container might get a signal from K8S that its time to go. But what will happen to all the processing that is being worked on in that time frame? Are we just descarding them? First of all we can, if we dont care about the stability of the service then why bother, but if we do we need to handle the termination "gracefully".

## Service Graceful Termination

In previous section we showed that we have multiple signals that can be used, and we used signal SIGTERM (15). That's exactly the signal that K8S will send the pod in order for it to fo graceful shutdown.

By graceful shutdown we mean that our service is not just being cut out of resources. Meaning that we have some time that we can use to cleanup, finish up ongoing tasks and exit at will.

So let's see how are we going to implement it in our node server:

```shell
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
We added code that listens on on process SIGTERM event code. When such even occurs, we're stopping the server but we're not just terminating it. Between the time that SIGTERM arrives and the specified `timeout` parameter, our server will refuse to accept any new requests and will wait before shutting down completely for the timeout duration.

We can test that statements! Start the server again:
```
$ ts-node ./index.ts
Server running at: http://0.0.0.0:3000, PID: 67116
```

Run cURL request and terminate the server:
```
$ curl -v http://0.0.0.0:3000/work
```
We should see that the request is finished and response sent back to the client - no more `(52) Empty reply from server` errors.
If we try access the server in the time frame between SIGTERM and actual shutdown we will get: 

```
$ curl -v http://0.0.0.0:3000/work
*   Trying 0.0.0.0:3000...
* connect to 0.0.0.0 port 3000 failed: Connection refused
* Failed to connect to 0.0.0.0 port 3000 after 3 ms: Connection refused
* Closing connection 0
curl: (7) Failed to connect to 0.0.0.0 port 3000 after 3 ms: Connection refused
```
Same error that you get if there's no server at all!


# Sending SIGTERM to K8S Pds
In K8S, whenever the Pod is terminated, it is sent a SIGTERM signal.
Similarly to the way we used `kill` command, we can terminate pods using `delete` command:
```shell
$ kubectl delete pod my-pod-qgldf
```
K8S will kill the Pod, the signal will be sent to the Docker container, that will be send to the running process, that's expected to handle the shutdown process.

Don't beleive me? Let's give it a go.

## K8S sample application

We're going to use the same Hapi server code, but containarize it and deploy on local k8s cluster:
```dockerfile
FROM node:19-bullseye
WORKDIR /app
COPY index.ts package.json package-lock.json /app/
RUN npm install
EXPOSE 3000
ENTRYPOINT ["/app/node_modules/.bin/ts-node", "index.ts"]

```
Build docker container:
```
$ docker build -t poc -f ./Dockerfile .

First let's run it and see if we can get SIGTERM related logs:
```
 docker run -t poc:latest
```
Get the docker id:
```shell
CONTAINER ID   IMAGE                  
86b0a46730ba   poc:latest             
```
And KILL it!
```shell
$ docker stop 86b0a46730ba
```
You should see the docker run command stoped with the following output:
```
$  docker run -t poc:latest
Server running at: http://0.0.0.0:3000, PID: 1
Received SIGTERM
Server stopped.
```
Now lets do some K8S on a local cluster.

First we need to load container image to the cluster (I'm using Kind):
```
```shell
$ kind load docker-image poc:latest --name my-cluster
```
K8S Deployment manifest:@
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

Delete the pod:
```
$ kubectl delete pod poc-deployment-bf749f576-wfmv9
```
Should get same results!

# Summary


In this article we covered SIGTERM signals, how they are used in Unix-based system as well as showed practical application of handling such 

# References

[1] https://nodejs.org/en/

[2] https://www.typescriptlang.org/

[3] https://www.docker.com/

[4] https://kind.sigs.k8s.io/

[5] https://www.npmjs.com/

[6] https://curl.se/

[7] https://kubernetes.io/