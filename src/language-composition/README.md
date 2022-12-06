# Programming Languages Composition
## Abstract
We have many Programming Languages (PL) today, each with its speciality and applications. But still, traditional software is written using a single programming language. We have tools and processes that allow language compositions, but these tools are cumbersome, slow and complex. This article will overview the Programming Language Composition (PLC) problem space and the applied composition techniques.

## Introduction
Realistic software is usually not bound to a single domain; storage, serialisation, network, security, concurrency, and domain-specific problems are just a few examples. Domain-Specific Languages (DSL) are built and optimised for specific problems. Some general-purpose languages are better suited for performance-intensive applications, while others have better readability or broader library support. 
Language composition can enable us to write software that is not restricted by single PL limitations, allowing us to take advantage of different language features flexibly.
Programming Language Composition (PLC)
It happened to me multiple times when I was writing software in one PL, and I needed some functionality or library available only in another PL. And I was stuck! Usually, I default to writing a library wrapper, invoking a CLI tool or some executable.
Generally, I see PLC as a solution for fast prototyping or experimentation with PL features. If we were not restricted in PL integration, we would be able to migrate legacy software in an iterative way, module by module, function by function. Or write domain logic in a readable PL (let's say Python) and implement the rest in a more performant language (Rust, for example)
Of course, having multiple languages in the same project creates more complexity. It requires engineers to understand and be proficient in more than one language. But if we put this cognitive load aside. And do some blue-sky thinking; why would we want to build restrictive software solutions locked in one paradigm or ecosystem? After all, we know that we will reach a dead end.
Some tools and processes allow language composition. However, it comes with runtime performance impact and operational costs, such as integration effort, and a need for better tooling, such as debugging and profiling.
Let's have an overview of these methods.

###  Distributed Applications (DA)
Distributed Applications (DA) is a highly adopted approach in the IT industry that addresses this problem. Usually referred to as Microservices Architecture these days, it doesn't have to be "micro" per se. DA are multiple components that act as one system, communicating over the networking protocol. Since the network layer abstracts components' internals, it allows these components to be implemented independently, using any language or framework, as long as it has a well-established communication protocol. However, it requires the application to be distributed and comes with significant infrastructure, networking and deployment complexity. Networking communication also comes with higher latency when compared to in-memory communication.

### Process-based - Fork, Exec, Spawn
With a Process-based approach, the software is composed of a collection of processes. One process can invoke another and create this relationship with internal abstraction on the Operating System (OS) level.
Fork - an operation where a process creates a copy of itself. 
Exec - an operation that runs an executable in an existing process context. 
Spawn - loads and executes a new "child" process. Where the "parent" process manages this "child". 

### Process-based - IPC
Yet another process-based approach is Inter-Process Communication (IPC). IPC is a mechanism that allows multiple processes on the same machine to communicate over the local network or memory. 
Shared Memory - Multiple processes communicate using the same memory. One produces data, and the other consumes it. This communication is self-explanatory.
Messaging Passing -Processes communicate without having shared a memory. Instead, they are using messages. Similarly to the traditional Client-Server model, the connection is established over the network, and messages are sent from one process to another.

### Foreign Function Interface (FFI)
Some of the most popular applications are built with a mix of languages. Firefox browser is one example; made with C/C++, Java, JavaScript and Python (maybe more). With this kind of project, it is a general practice to write an application base system in low-level programming languages and extend it with high-level languages. The main reason for adopting multiple programming languages is the code reuse of existing software; another is to take advantage of language-specific features [7].
Multi-language projects face a challenge where each language has separate runtimes and address space. Therefore, there must be an established cross-process communication protocol for these programs to interact. SWIG [1] is a popular tool that addresses such challenges. SWIG is an interface compiler that links programs written in C/C++ with other languages; it is based on messaging and type conversion via a Foreign Function Interface (FFI), which can be cumbersome and costly in runtime performance.
Another operational overhead with SWIG-like tools is that cross-interactions require the definition of interface files [2], which are foreign to the interfacing languages; to integrate with SWIG, one must understand SWIG protocol and its toolchain. It has a steep learning curve and creates another barrier to its adoption.

### Interpreters-based composition
Another approach to composing multiple languages in one software solution is interpreter-based composition. If we have multiple interpreters written in the same language for different PLs, we can potentially compose them in the same address space. However, it comes with challenges, such as Language Semantics. The Unipycation case study [3] shows that a meta-tracing technique is feasible to create an efficient Virtual Machine (VM) cross-language composition. Unipycation composes multiple RPython interpreters; PyPy [4] and Pyrolog [5]. Both components have their own meta-tracing JIT, which enables a similar runtime performance when the composed outcome is compared to stand-alone interpreters.
One challenge with the Unipycation and the PyPy toolchain is the long translation process of converting the RPython program to an executable artefact. [9] This creates long-time feedback iterations when developing and creates another obstacle in language composition adaptation.
Another challenge that the Unipycation case study showed is semantic friction; composed languages usually have different semantics. Semantic differences make languages specific optimisation hard [3].
Eco [7] language composition editor provides the fill of an ordinary text editor while using syntax-direct editing behind the scenes. Unlike traditional editors, Eco operates and saves ﬁles as tree structures rather than as a conventional text-based source ﬁles. This again creates friction when integrating with widely adopted engineering practices and tools, such as version control systems that are usually text-based.
The Unipycation case study shows that it can compose multiple languages without compromising performance with minimal engineering effort. But after all, this case study is experimental and not Production Ready in any form.

### Web stack (TODO)

We have JavaScript (JS), the primary language that runs in our browsers. Recently we had a new addition - WebAssembly (WASM), but still JS is the de-facto language of the web.
JS isn't perfect, so we came with all these supersets, transpired languages that we can write with strong types (TypeScript), or Haskell-like functional programming (ELM), but then we compile them and we run JS in our prowsers.
What will happen if I have a library written, let's say, in CoffeeScript, and I want to import it to my TypeScript (TS)application? I need to compile it to JavaScript first inorder to use it TS. And what if I wanted to use three different libraries from three different PL? I would need to go through the same transpile process over and over again. 

## Summary (TODO)

References
[1] Beazley, D.M., 1996, July. SWIG: An Easy to Use Tool for Integrating Scripting Languages with C and C++. In Tcl/Tk Workshop (Vol. 43, p. 74).
[2] "SWIG. 4.0 Documentation." SWIG, https://swig.org/Doc4.0/SWIGDocumentation.html#Introduction_nn5. Accessed 9 October 2022.
[3] Barrett, E., Bolz, C.F. and Tratt, L., 2013, October. Unipycation: A case study in cross-language tracing. In Proceedings of the 7th ACM workshop on Virtual machines and intermediate languages (pp. 31–40).
[4] Rigo, A. and Pedroni, S., 2006, October. PyPy's approach to virtual machine construction. In Companion to the 21st ACM SIGPLAN symposium on Object-oriented programming systems, languages, and applications (pp. 944–953).
[5] Bolz, C.F., Leuschel, M. and Schneider, D., 2010, July. Towards a jitting VM for prolog execution. In Proceedings of the 12th international ACM SIGPLAN symposium on Principles and practice of declarative programming (pp. 99–108).
[6] Diekmann, L. and Tratt, L., 2013, June. Parsing composed grammars with language boxes. In Workshop on Scalable Language Specifications.
[7] Grichi, M., Abidi, M., Jaafar, F., Eghan, E.E. and Adams, B., 2020. On the impact of interlanguage dependencies in multilanguage systems empirical case study on java native interface applications (JNI). IEEE Transactions on Reliability, 70(1), pp.428–440.
[8] Diekmann, L. and Tratt, L., 2014, September. Eco: A language composition editor. In International Conference on Software Language Engineering (pp. 82–101). Springer, Cham.
[9] Barrett, E., Bolz, C.F. and Tratt, L., 2015. Approaches to interpreter composition. Computer Languages, Systems & Structures, 44, pp.199–217.
[10] Fowler, M. and Foemmel, M., 2006. Continuous integration.
[11] Humble, J. and Farley, D., 2010. Continuous delivery: reliable software releases through build, test, and deployment automation. Pearson Education.
[12] Cohn, Mike. Succeeding with Agile. Chennai Inde, Pearson, 2010, pp. 311–316. Accessed 02 Mar. 2022.