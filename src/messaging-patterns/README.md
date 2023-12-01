# Messaging Patterns in Distributed Systems

# Abstract

In this article, we're going to cover multiple communication patterns in distributed systems. This is by far not an exhaustive list of all the patterns, but if should give a good understanding of the problem space and different approaches to communication design in cloud-based systems.

# Introduction

In distributed systems, we're dealing with multiple machines that communicate with each other over a netwtork. This communication can be done directly via one of the networking protocols or via messaging layers.
There are different ways to architect communication in distributed systems. We're going to review these patterns on a high level and list their advantages and shortcomings.

# Messaging Patterns

## Request-Reply (Request-Response)

In this pattern, the client sends a request to a server and waits synchronously for a response. 
This pattern is the most common use as it involves only networking and no additional infrastructure.
The actual communication protocol can vary. The HTTP and RPC are just a few examples.

Here, both parties; Client and Server are tightly coupled, i.e. there is no abstraction between them.


Advantages:

This pattern is one of the most straightforward ones. Its easy to understand and implement.

Disadvantages:
It has limitted scalability. Point-to-point messaging may not scale well for large systems with many producers and consumers. It creates fragile, tight coupling between distrebuted components.


## Message Queue (Point-to-Point or Direct-Messaging):

Here messages are sent from a single producer to a single consumer. 
The most common way to implement this pattern is using a messaging queue. 
Point-to-point messaging is well-suited for applications where each message must be processed by a single consumer. It's simple and effective but it's limited in its scalability.


## Publish-Subscribe (Pub/Sub):

In the publish-subscribe pattern, messages are sent from a producer (publisher) to multiple consumers (subscribers) via topic. Subscribers subscribe to the topic in order to receive the messages. 
This pattern is particularly useful for applications that need information dissemination to multiple recipients.
The topic acts as an abstraction layer between the Publishers and the Subscribers.


## Event Sourcing 

This pattern is useful for systems that need to maintain an auditable and traceable history of changes.
Event sourcing involves capturing all changes to a system state as a sequence of events. 
The events are stored persistently for a duration of time and can be used to reconstruct the application state at any point in time. 

## Fan-Out / Fan-In

In this pattern, a message is sent to multiple consumers, and the responses from these consumers are aggregated before being sent back to the original sender. 
This pattern is useful when dealing with highly distributed workloads, and the results must be collected and combined.


## Dead Letter Queue (DLQ):

This pattern is used to handle messages that cannot be processed by dedicated consumers, which can be for a variety of reasons. 
These failed-to-process messages are sent to a dedicated dead letter queue, where they can be analyzed, monitored or reprocessed in one way or another. 
This pattern ensures that problematic messages do not disappear from the system and do not block the processing of other messages in the system

## Saga

The saga pattern is used for managing long-lived transactions in a distributed system. 
The main idea is to break down a large transaction into a series of smaller, independent transactions called "saga steps".
Each step represents a distinct operation or action within the overall transaction. Importantly, each step is associated with a compensating transaction that can be executed in case of failure, ensuring that the system can be brought back to a consistent state.

The Saga pattern is particularly useful in microservices architectures, where different services collaborate to fulfill business processes.


Implementing sagas requires careful consideration and planning involving coordination mechanisms, and error-handling strategies to ensure the reliability of the overall system. It's very complex in its nature.

# Summary
TODO...