# Consistent Hash Ring

## Abstract

In this article, we will cover the theory of Consistent Hashing and how it's used in distributed systems.
There will be no code, just a few diagrams and explanations.

# Introduction

...
In distributed applications, we can scale vertically by adding more resources to a machine instance or horizontally - by adding more machines.

Generally, horizontal scaling is preferred since there are fewer physical limitations when compared to vertical scaling. But Horizontal Scaling comes with its challanges.

Consistant Hashing is a common technique to achieve this goal.

## The Problem

Let's say we have `n` db servers, i.e server pool of size `n`.
How can we distribute evenly the load of incoming requests between them?

One way to do so is to use some kind of hash method like:
```
server = hash(key) % n
```

Sounds easy enough.

But this approach won't work well when we the size of the server pool is inconsistant or when the data distributed un-evenly acrross `n` instances.

The problem arises when the size of the server pool is dynamic. What will happen when a new server is added or some servers are removed from our server pool?
If we start with `n` servers and we use `hash(key) % n` and then one of the machines is going down and suddenly we need to perform `hash(key) % n - 1` - But that won't work since the number of servers went down by one. Also, what will happen to the data on the server that went down?

## Consistant Hashing

Consistent hashing is a hushing algorithm that when a hash table is resized, on average only `number of keys / number of slots` keys needs to be remapped. If we compare that to traditional hash tables, a change in the number of slots results in almost all keys to be remapped.


## Hash space and hash ring

We're going to use SHA-1 for our hash function f. SHA-1 hash space range is between 0 and 2^160-1.

```
x0 = 0
xn = 2^160-1

[x0, ..., xn]
```

So that's our hash space. To get the hash ring we connect both ends to get a circle.

## Hash servers

Using the same hash function f, we map servers on the ring using their IP:

Servers: S0, S1, S2, S3

[x0, ..., S0, ..., S1, ..., S2, ..., S3, ...,xn]


## Hash Keys

Hash function used here would be different from the one used before and there will be no modular operations.

Keys: k0, k1, k2, k3,

[x0, ..., S0, ...., k1, ..., S1, ..., k2, ..., S2, ..., k3, ..., S3, ..., k0, ..., xn]

## Server Lookup

To determine which server has the key, we go clockwise from the position of the key until we find the server.

In our example:
k0 - S1
k1 - S2
k2 - S3
k3 - S4

## Adding a server

Let's now add a server S4. This will require us to redistribute a fraction of the keys.
For example:

Given:

Servers: S0, S1, S2, S3
Keys: k0, k1, k2, k3,
[x0, ..., S0, ...., k1, ..., S1, ..., k2, ..., S2, ..., k3, ..., S3, ..., k0, ..., xn]

Adding S4:

Servers: S0, S1, S2, S3
Keys: k0, k1, k2, k3,

[x0, ...,S4, ..., S0, ...., k1, ..., S1, ..., k2, ..., S2, ..., k3, ..., S3, ..., k0, ..., xn]

We added S4 before S0, therefore the only thing we need to change is to more k0 to S4. Nothing else.

## Removing a server

Let's now remove a server from the ring.
Remote: S3

Servers: S0, S1, S2
Keys: k0, k1, k2, k3,

[x0, ...,S4, ..., S0, ...., k1, ..., S1, ..., k2, ..., S2, ..., k3, ..., ..., k0, ..., xn]

All we need is to re-map k3 to be stored at S4.

## Known Issues

It is impossible to keep the same size of partitions on the ring for all the servers when a server can be added or removed.
Partition is the hash space between adjacent servers.

It is possible that the size of the partitions on the ring assigned to each server is very small or very large...

It is also possible that some servers will have most of the data assigned to them when other servers will have no data...

But we have an answer to that problem as well! Virtual nodes!

## Virtual nodes

Each actual server is represented by a number of virtual nodes:

Let's choose the number 3 as the number of virtual nodes:

Servers: S0, S1
Virtual Nodes: S0_0, S0_1, S0_2, S1_0, S1_1, S1_2


With virtual nodes, each server is responsible for multiple partitions. Labels with S0_ are managed by S0 server.
And our virtual nodes will be placed in segregation:

The ring:
[ ..., S1_0,..., S0_0,..., S1_1,..., S0_1,..., S1_2,..., S0_2 ]

To find where a key is located, we do the same process going clockwise untill we get to the virtual node and then identify which actual server it belongs to.


With this approach, as the number of virtual nodes increases the distribution of keys becomes more balanced. 

However, it also requires additional storage since we need to store the data about the virtual nodes - that's a tradeoff.



## Gains

Minimal key redistribution when servers are added or removed.

Easier to scale horizontally, since data is distributed evenly across nodes in the ring

Avoid the hotspot key problem - Consistent hashing addresses this problem by distributing the keys across the nodes in a balanced way

## Realworld usage

AWS DynamoDB - https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf

Apache Casandra - https://www.cs.cornell.edu/projects/ladis2009/papers/lakshman-ladis2009.pdf

# Summary

We discussed the consistent-hashing technique and its application in distributed systems.

This writeing was for my own sake of understanding and organising my thoughts as it was about knowledge sharing. I hope it was helpful. If you have questions/objections/observations/complaints, don't hesitate to reach out!


References:
https://tom-e-white.com/2007/11/consistent-hashing.html
