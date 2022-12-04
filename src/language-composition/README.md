
# Abstract

Traditional software is written in a single programming language. Although some tools and processes allow runtime language compositions, these tools are cumbersome, slow and complex. This document will present a research proposal in the programming language composition domain. First, I will overview the problem space and provide background to the state of the given problem. Finally, I will present my research goals and the practical methods I intend to use to achieve them.
Introduction

Traditional software is written using a single programming language. However, realistic software scope is usually not bound to a single domain; storage, serialisation, network, security, concurrency, and domain-specific problems are just a few examples. Some general-purpose languages are better suited for performance-intensive applications, while others have better readability and a developer-friendly experience. In contrast, domain-specific languages (DSL) are built and optimised for specific problems. Therefore, there is a benefit to multi-language software composition where we can leverage each language's benefits.
Microservices architecture is a highly adopted approach in the IT industry that addresses this problem. Microservices are distributed applications; multiple components act as one system, communicating via a networking protocol. Since the network layer abstracts components’ communication, it allows system components to be implemented independently, using any language. However, it requires the application to be distributed and comes with significant infrastructure, networking and cognitive complexity.
Another approach to composing multiple languages in one software solution is language composition. Language composition allows the use of various languages in one solution leveraging the benefits of language-specific features without the overhead of distributed systems. 
There are tools and processes today that allow language composition. However, it comes with runtime performance impact and operational costs, such as integration effort, and a need for better tooling, such as debugging and profiling.
In this document, I propose to research programming language composition, the process of composition, composed runtime performance and its practical application.


# Background
Some of the most popular applications are built with a mix of languages. Firefox browser is one example; made with C/C++, Java, JavaScript, Python and Rust. It is a general practice to write an application base system in low-level programming languages and extend it with high-level languages. The main reason for adopting multiple programming languages is the code reuse of existing software; another is to take advantage of language-specific features [7].

Multi-language projects face a challenge where each language has separate runtimes and address space. Therefore, there must be an established cross-process communication protocol for these programs to interact. SWIG [1] is a popular tool that addresses such challenges. SWIG is an interface compiler that links programs written in C/C++ with other languages; it is based on messaging and type conversion via a Foreign Function Interface (FFI), which can be cumbersome and costly in runtime performance. 

Another operational overhead with SWIG-like tools is that cross-interactions require the definition of interface files [2], which are foreign to the interfacing languages; to integrate with SWIG, one must understand SWIG protocol and its toolchain. It has a steep learning curve and creates another barrier to its adoption.

Programming language composition allows for a straightforward language combination. However, it comes with challenges, such as the interaction of language semantics and performance implications. The Unipycation case study [3] shows that creating an efficient Virtual Machine (VM) cross-language composition is feasible by utilising a meta-tracing technique. Unipycation composes multiple RPython interpreters; PyPy [4] and Pyrolog [5]. Each component has its own meta-tracing JIT, which enables a similar runtime performance when the composed outcome is compared to stand-alone interpreters. This approach of composition relies on RPython and provided JIT functionality. Alternative non-RPython VM-based compositions are also viable as long as a common VM can be used to connect the languages.

One challenge with the Unipycation and the PyPy toolchain is the long translation process of converting the RPython program to an executable artefact. [9] This creates long-time feedback iterations when developing and creates another obstacle in language composition adaptation.

Another challenge that the Unipycation case study showed is semantic friction; composed languages usually have different semantics. Semantic differences make languages specific optimisation hard [3].

Eco [7] language composition editor provides the fill of an ordinary text editor while using syntax-direct editing behind the scenes. Unlike traditional editors, Eco operates and saves ﬁles as tree structures rather than as a conventional text-based source ﬁles. This again creates friction when integrating with widely adopted engineering practices and tools, such as version control systems that are usually text-based.

# Language Composition

A computer program written in multiple programming languages


# Why?
✅ Fast prototyping and experimentation with PL features
✅ Non-restrictive software solutions, paradigm-lock
✅ Iterative migration of legacy systems 
✅ Remove integration performance impact
✅ Reduce integration complexity

# Programming Languages (PL) Today
✅ PL paradigms - Functional, OOP, Logic, DSL, ...
✅ PL qualities - Static, Dynamic, Strongly Typed, Readability, Ecosystem …
✅ PL work great in a native environment

❌ PL composition
❌ PL adaptation
❌ PL feature cherry-picking


# Composition Solutions
✅ Distributed - MA, SOA, EDA
✅ Process based - IPC, Fork–Exec
✅ Foreign Function Interface (FFI)
✅ Language Composition - RPython

❌ Operational Overhead 
❌ Cognitive load
❌ Developer Experience (DX)
❌ Limited Language Support

# RPython interpreters composition - Syntax Directed Editing + Meta Tracing

✅ Performance (MT)
✅ Language agnostic JIT optimisations (MT)
✅ Relatively little engineering effort (MT)
✅ Traditional editor experience (SDE)

❌ Restricted to RPython interpreters (MT)
❌ Long feedback time (MT)
❌ Semantic Friction (MT)
❌ Integration with existing tooling (SDE) 

# Transpiled composition





# Background

# References

[1] Beazley, D.M., 1996, July. SWIG: An Easy to Use Tool for Integrating Scripting Languages with C and C++. In Tcl/Tk Workshop (Vol. 43, p. 74).
[2] “SWIG. 4.0 Documentation.” SWIG, https://swig.org/Doc4.0/SWIGDocumentation.html#Introduction_nn5. Accessed 9 October 2022.
[3] Barrett, E., Bolz, C.F. and Tratt, L., 2013, October. Unipycation: A case study in cross-language tracing. In Proceedings of the 7th ACM workshop on Virtual machines and intermediate languages (pp. 31-40).
[4] Rigo, A. and Pedroni, S., 2006, October. PyPy's approach to virtual machine construction. In Companion to the 21st ACM SIGPLAN symposium on Object-oriented programming systems, languages, and applications (pp. 944-953).
[5] Bolz, C.F., Leuschel, M. and Schneider, D., 2010, July. Towards a jitting VM for prolog execution. In Proceedings of the 12th international ACM SIGPLAN symposium on Principles and practice of declarative programming (pp. 99-108).
[6] Diekmann, L. and Tratt, L., 2013, June. Parsing composed grammars with language boxes. In Workshop on Scalable Language Specifications.
[7] Grichi, M., Abidi, M., Jaafar, F., Eghan, E.E. and Adams, B., 2020. On the impact of interlanguage dependencies in multilanguage systems empirical case study on java native interface applications (JNI). IEEE Transactions on Reliability, 70(1), pp.428-440.
[8] Diekmann, L. and Tratt, L., 2014, September. Eco: A language composition editor. In International Conference on Software Language Engineering (pp. 82-101). Springer, Cham.
[9] Barrett, E., Bolz, C.F. and Tratt, L., 2015. Approaches to interpreter composition. Computer Languages, Systems & Structures, 44, pp.199-217.
[10] Fowler, M. and Foemmel, M., 2006. Continuous integration.
[11] Humble, J. and Farley, D., 2010. Continuous delivery: reliable software releases through build, test, and deployment automation. Pearson Education.
[12] Cohn, Mike. Succeeding with Agile. Chennai Inde, Pearson, 2010, pp. 311–316. Accessed 02 Mar. 2022.
