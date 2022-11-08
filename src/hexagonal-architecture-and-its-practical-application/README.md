
# Hexagonal Architecture and Its practical application
# Abstract


# Introduction

Hexagonal architecture (HA) (aka ports and adapters architecture), is a software architectural pattern. HA is an alternative to the traditional layered architecture. HA divides a system into several loosely-coupled components, placing the application domain (aka application core) at its center. 

# The problem

The problem with layered architecture is that often we tend to take the separation of layers in such architecture lightly. In layered applications, the domain logic leaks between boundaries. Projects without well-defined domains are hard to reason about and maintain. Additionally, in large projects, the architecture is usually quite complex and it's hard to project it into one-dimensional layer drawings. 

In HA we don't have this dimensional restriction. We have the application on the inside communicating using abstraction over interfaces with the external world. Strict boundaries, abstraction and separation of concerns are
the key here.


# HA components

## Ports
HA components are connected through a Port. Port can be anything that has a functionality contract, but for simplicity, we will stick to interfaces. 

The term “Port" is inspired by the operating system (OS) concepts, where any device that respects the protocol of a port can be connected. For example Universal Serial Bus (USB).

## Adapters
Adapters are the layer between HA components and the outside world. Adapters will implement Port specifications and provide the contract fulfillment. There might be multiple adapters for any one port. 

The Adapter OOP design pattern is one of the well-known design patterns from the "Design Patterns" book. At its core, it "adapts" two objects by converting one interface of a class into another. The HA Adapter pattern is a particular use of this pattern.

## Domain (aka core)
This is the application domain. That's where we keep the business logic. The way I think about it; domain code should be easily explained to a non-technical person. For example, your product manager.


## Why is it called Hexoganal? Where is the Hex?

Funny story. Hexagonal might sound like there are 6 components in HA, but we counted only 3! If anything it should've been called Triangular Architecture. The term "Hexagonal" was used not because of the number six significance. The intention was to allow architectural drawings with enough room to insert ports and adapters as needed and not be constrained by a one-dimensional layered drawing.


# HA Key features

## Dependency Inversion (DI)
In HA we have a strong use of interfaces, naturally, it inverts the the dependencies. With DI our component dependencies are not dependent on other component implementations, instead, they are dependent upon abstraction.
With DI, we leverage decoupling, modularisation and ease of testing.

“High-level modules should not depend on low-level modules. Both should depend on abstractions. Abstractions should not depend on details. Details should depend on abstractions.” Dependency Injection by Martin Fowler.

## Separation of Concerns 

Our application layers are isolated and have clear responsibilities and definitions.

## Domain-Driven
The application core is agnostic of technologies and infrastructure. It's easier to understand and grasp the business requirements and their changes when the application domain is well-defined and isolated. It can also be built and tested in isolation.


# Practical Application of HA








# References



# References
[1] https://alistair.cockburn.us/hexagonal-architecture/