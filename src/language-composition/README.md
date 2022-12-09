# Programming Languages Composition

## Abstract

There is a variety of Programming Languages (PL) today, each with its speciality and features. But still, traditional software is written using a single programming language. We have tools and processes that allow language compositions, but these tools are cumbersome, slow and complex. This article will overview the Programming Language Composition (PLC) problem space and the applied composition techniques.

## Introduction

Realistic software is usually not bound to a single domain; storage, serialisation, network, security, concurrency, and domain-specific problems are just a few examples. Domain-Specific Languages (DSL) are built and optimised for specific problems. Some general-purpose languages are better suited for performance-intensive applications, while others have better readability or broader library support.

PLC is a computer program written in multiple programming languages. PLC can enable us to write software that is not restricted by single PL limitations, allowing us to take advantage of different language features flexibly.

## Programming Language Composition (PLC)

It happened to me multiple times when I was writing software in one PL, and I needed some functionality or library available only in another PL. And I was stuck! Usually, in that case, I default to writing some wrapper (aka glue code), invoking a CLI tool or some executable.

I see PLC as a solution for fast prototyping and experimentation with PL features. If we were not restricted by PL integration constraints, we would be able to migrate legacy software in an iterative way, module by module, function by function. Or write domain logic in a more readable PL (let's say Python) and implement the rest in a more performant language (for example, Rust).

Of course, combining multiple languages in the same project creates more complexity. It requires engineers to understand and be proficient in more than one language. But if we put this cognitive load aside. And do some blue-sky thinking; why would we want to build restrictive software solutions locked in one paradigm or ecosystem? After all, we know that we will reach a dead end.

Some tools and processes allow language composition. However, it comes with runtime performance impact and operational costs, such as integration effort, and a need for better tooling for tasks such as debugging and profiling.

Let's have an overview of these methods.

### Distributed Computing

[Distributed Computing](https://en.wikipedia.org/wiki/Distributed_computing) [1] is a highly adopted approach in the IT industry that addresses this problem. Usually referred to as Microservices Architecture these days, but it doesn't have to be "micro" per see to be distributed. Distributed systems consist of multiple components that act together, communicating over the networking protocol. Since the network layer abstracts components' internals, it allows these components to be implemented independently, using any language or framework, as long as it has a well-defined communication protocol. However, it requires the application to be distributed and comes with significant infrastructure, networking and deployment complexity. Networking communication also comes with higher latency when compared to in-memory communication.

### Process-based - Fork, Exec, Spawn

[Fork–exec](https://en.wikipedia.org/wiki/Fork%E2%80%93exec) [2] is a Unix Operating System (OS) technique where an executing process spawns a new program. With a Process-based approach, the solution is composed of a collection of processes. One process can invoke another and create this relationship with internal abstraction on the OS level.

Fork - an operation where a process creates a copy of itself

Exec - an operation that runs an executable in an existing process context

Spawn - loads and executes a new "child" process. Where the "parent" process manages this "child"

### Process-based - IPC

Yet another process-based approach is Inter-Process Communication (IPC). IPC is a mechanism that allows multiple processes on the same machine to communicate over the local network or memory.

Memory - Processes communicate using the same memory. One produces data, and the other consumes it.

Messaging - Processes communicate without shared memory. Instead, they are using messages. Similarly to the Client-Server model, the connection is established over the network, and messages are sent from one process to another.

### Foreign Function Interface (FFI)

Some of the most popular applications are built with a mix of languages. Firefox browser is one example; made with C/C++, Java, JavaScript and Python (maybe more). With this kind of project, it is a general practice to write an application base system in low-level programming languages and extend it with high-level languages. The main reason for adopting multiple programming languages is the code reuse of existing software; another is to take advantage of language-specific features [3].

Multi-language projects face a challenge where each language has separate runtimes and address space. Therefore, there must be an established cross-process communication protocol for these programs to interact. [SWIG](https://www.swig.org/papers/Tcl96/tcl96.html) [4] is a popular tool that addresses such challenges. SWIG is an interface compiler that links programs written in C/C++ with other languages; it is based on messaging and type conversion via a Foreign Function Interface (FFI), which can be cumbersome and costly in runtime performance.

Another operational overhead with SWIG-like tools is that cross-interactions require the definition of interface files [5], which are foreign to the interfacing languages; to integrate with SWIG, one must understand SWIG protocol and its toolchain. It has a steep learning curve and creates another barrier to its adoption.

### Interpreters-based composition

Another approach to composing multiple languages as one solution is interpreter-based composition. If we have multiple interpreters written in the same language for different PLs, we can compose them in the same address space. However, it comes with challenges, such as Language Semantics. The [Unipycation](https://soft-dev.org/pubs/pdf/barrett_bolz_tratt__unipycation_a_study_in_cross_language_tracing.pdf) [6] case study shows that it is feasible to create an efficient Virtual Machine (VM) cross-language composition.

Unipycation composes multiple RPython interpreters; [PyPy](https://www.pypy.org/) [7] and [Pyrolog](https://www.bibsonomy.org/bibtex/2f58dd8a58d274eab26c1eaf6f501e493/gron?lang=de) [8]. Both components have their own [meta-tracing](https://tratt.net/laurie/research/pubs/html/bolz_tratt__the_impact_of_metatracing_on_vm_design_and_implementation/) [10] JIT, which enables a similar runtime performance when the composed outcome is compared to stand-alone interpreters.

[Eco](https://soft-dev.org/pubs/html/diekmann_tratt__eco_a_language_composition_editor/) [10] editor provides the fill of an ordinary text editor while using syntax-direct editing behind the scenes. Unlike traditional editors, Eco operates and saves ﬁles as tree structures rather than as a conventional text-based source ﬁles. This again creates friction when integrating with widely adopted engineering practices and tools, such as version control systems that are usually text-based.

The Unipycation case study shows that it can compose multiple languages without compromising performance with minimal engineering effort. But after all, this case study is experimental and not Production Ready in any form.

## JS-Transpiled Web

JavaScript (JS) is the primary language that runs in our browsers. Recently we had a new addition of WebAssembly (WASM), but still, JS is the de-facto language of the web. JS has its limitations. So we came up with different js-transpired languages to address them, enabling us to write with strongly-typed code (TypeScript) or Haskell-like functional programming (ELM). Still, they all transpile into JS that runs in our browsers.

What will happen if we have a project written, let's say, in CoffeeScript, and we need to import a TypeScript (TS) library? We will need to convert our script to JS and import it. What if we use three different libraries implemented in three PLs? We would need to go through the same transpile process over and over again.

This is another place for the PLC application that can be a good fit.

## Summary

In this article, we overviewed the problem of Programming Language Composition (PLC). As far as my research goes, I've enumerated all the main methods we have today for writing multi-lingual software.

The state of language composition and integration today is far from ideal, it's cumbersome and convoluted in my opinion, and I hope we see some improvements in that domain soon. We have solutions that work, and we keep on using them no matter the cost, without re-evaluating them. What we need is precisely that re-evaluation.

This write-up is for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. It's based on my recent research proposal for a part-time PhD. I hope it was helpful.

If you have questions/objections/observations/complaints, don't hesitate to reach out!

## References

[1] https://en.wikipedia.org/wiki/Distributed_computing

[2] https://en.wikipedia.org/wiki/Fork%E2%80%93exec

[3] https://ieeexplore.ieee.org/document/9246706

[4] https://www.swig.org/papers/Tcl96/tcl96.html

[5] https://swig.org/Doc4.0/SWIGDocumentation.html#Introduction_nn5

[6] https://soft-dev.org/pubs/pdf/barrett_bolz_tratt__unipycation_a_study_in_cross_language_tracing.pdf

[7] https://www.pypy.org/

[8] https://www.bibsonomy.org/bibtex/2f58dd8a58d274eab26c1eaf6f501e493/gron?lang=de

[9] https://tratt.net/laurie/research/pubs/html/bolz_tratt__the_impact_of_metatracing_on_vm_design_and_implementation/

[10] https://soft-dev.org/pubs/html/diekmann_tratt__eco_a_language_composition_editor/
