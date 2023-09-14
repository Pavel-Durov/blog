# Developer Experience and SDKs
## Abstract.

This article delves into SDKs (Software Development Kit), covering their development, maintenance, and the crucial aspects of DX (Developer Experience). We'll explore DX core principles with TypeScript examples and examine code evolution and compatibility concerns.

## Introduction

SDK provides a means of integration with external systems such as remote APIs (Application Programming Interface), local ABIs (Application Binary Interfce), devices or hardware platforms. It is a collection of software components bundled as one package.
This package contains everything necessary to effectively use the underlying system for which the SDK provides functionality.
But it's not enough to have an SDK that is functional, if we want the SDK to be adopted and survive the test of time, it also needs to have a good user experience. We call this experience DX since developers are the main users of SDKs.

## Why build SDK?

SDKs offer a streamlined approach to crafting applications for specific targets. They function as specialised toolkits. One of their key benefits lies in simplifying the integration process. This simplification is achieved by often hiding the complexities of internal implementation and providing an intuitive interface.

Additionally, SDK is a reusable component. It allows seamless integration into multiple projects, reducing code duplication, and facilitating support and maintenance.

## SDK vs API

API (Application Programming Interface) expose the internal functionality of a system without exposing its internals in a language-agnostic fashion.
Distinctively, SDKs are tailored to specific programming languages, while APIs maintain a higher level of abstraction. This distinction makes SDKs more user-friendly and readily adoptable due to their straightforward integration and developer experience.
Usually, SDKs use some sort of API behind the scenes while enhancing it with additional functionality, comprehensive documentation, and practical examples.


## DX

DX (Developer Experience) describes the interactions and experience that a software developer has when working with a tool or a piece of code. 
If you are familiar with the term UX (User Experience), then you can think of DX in the same terms where the user is the developer.
It might be subjective, but great DX is hard to deny. 
When evaluating DX we should consider multiple factors.

### DX - Explicit functionality


While this principle may appear elementary, it's essential and sometimes overlooked.
A tool should precisely do what it claims to do. Surprisingly, numerous tools are inclined to do things that a developer would not reasonably anticipate.
Consider this scenario: You've integrated an SDK into your project inorder to use some kind of remote Restful API. Yet, upon its use, it unexpectedly generates hefty files on your disk due to an unexpected optimisation process that was never mentioned.

### DX - Comprehensive documentation

Documentation does not need not be verbose, but it should be precise. Crafting clear documentation is one of the most challenging parts in software engineering. 
It is crucial for documentation to remain up-to-date, striking a balance between brevity and comprehensiveness.

### DX - Intuitive and easy to use

It should be intuitive. A developer should look at the code and immediately understand how to work with it without the need for extensive documentation exploration.
When tailored to a specific programming language, it should faithfully stick to the language's conventions and avoid unnecessary deviations. The code's appearance should be familiar and approachable.
The end-to-end use of the tool should be easy as well. That includes installation, configuration and actual use.
### DX - Adaptability

It should be designed to be flexible and adaptable. That includes modularity, configuration options and version management.
### DX - Compatibility

In order to achieve good DX, software needs to be designed with compatibility in mind.
The worse DX is when you upgrade your SDK version and suddenly you need to fix all the places that it's used in the project.
We will talk more extensively about compatibility types and examples later on.

### DX - Quickstarts and Samples

Compact, functional examples that provide a comprehensive glimpse of the tool's capabilities are priceless. They trigger those "AHA" moments when, upon using the provided sample, everything effortlessly falls into place.
One of the best quickstarts I've seen is is node.js express:

```js
const express = require('express')
const app = express()
const port = 3000

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})
```

In just 11 lines, we can get a server up and running. The first time I've seen it I was blown away.

## Node.js and TypeScript SDK

Let's talk about TypeScript SDK specificly.
To deliver a good DX we need first to understand the client. We need to ask - What do TypeScript engineers expect from the SDK?

To name a few of these expectations:
- Easy-to-use
- Promises and Async/Await - async functionality by default.
- Package manager support -  installation with one of the goto package managers like npm
- Functional code examples - copy, paste, execute.
- Type Definitions -TypeScript is a statically-typed language, types are treated as a basic component.
- Type Safety - type safety should be enforced throughout the interfaces.
- Modules Support - compatibility with modern module systems like CommonJS and ES6 modules
- Optional parameters - Optional parameters enhance the flexibility and usability of the SDK.
  
In the following examples, we will try to address most of these points, with a focus on optional parameters and code evolution.

## Example

Let's say we have the following Restful API

```shell
POST /posts - Creates new post
PUT /posts/{id}/like - Like a post
```

If I translate these endpoints to TypeScript SDK, it would be something like:

```ts
import Posts from 'posts';
const posts = new Posts();
const post = await posts.createPost('title', 'content')
await posts.like(post.id);
```

Let's call it V1.

## Optional parameters and code evolution

Let's talk about optional parameters and how they affect code evolution over time.


Consider our SDK `createPost` function:
```ts
function createPost(title: string, content: string): Promise<Post> { /* ... */}
```

Let's say we want to allow multiple ways of creating Post objects. And the obvious tool of choice for this job is, you guessed it right - optional parameters.

But we don't want to break the current usage of this function, we want to introduce new functionality while keeping the SDK compatible with previous versions.
Here's how we can add new functionality:

```ts
function createPost(title: string, content: string, subtitle?: string): Promise<Post> { /* ... */}
```
Now we can use it in both ways:

```ts
import Posts from 'posts';
const posts = new Posts();
const post = await posts.createPost('title', 'content');


createPost("My Title", "My Content");
createPost("My Title", "My Content", "My Subtitle");
```

It's already morphing into something weird. 
Intuitively I would expect the title to be the first function argument, followed by the subtitle and then the content. But we can't just change the order at will, we will be breaking V1 compatibility. If we did, it will mean that for V1 suddenly all the content will be set as subtitles - that is unacceptable.

And what will happen when we add another parameter to our function?

```js
function createPost(title: string, content: string, subtitle?: string, date?: Date): Promise<Post>{ /* ... */ }
```
Now this function can be used as:

```js
createPost("My Title", "My Content");
createPost("My Title", "My Content", "My Subtitle", new Date());
```
But also as:
```js
createPost("My Title", "My Content", undefined, new Date());
```
Which is also not great. Looking at the code, it's hard to understand what is set as `undefined`.

So, what would be better to use in this case?

We can use an object for that!

```ts
interface Params {
  title: string;
  subtitle?: function;
  content?: string;
  date?: Date;
}
function createPost(params: Params) : Promise<number> { /* ... */ }

```

And we can use it as:

```ts
await createPost({
  title: "My Title", , 
  content: "My Content", 
});
await createPost({
  title: "My Title",
  subtitle: "My Subtitle",
  content: "My Content", 
});
await createPost({
  title: "My Title", 
  subtitle: "My Subtitle", 
  content: "My Content", 
  date: new Date()
});
```

Which is more readable, has no specific parameter ordering, and most importantly, no breaking changes.
It's easier to evolve the functionality based on types rather than function parameter orders.

Next, we'll overview compatibility which is also a very important topic when it comes to maintaining and evolving software over time.

# Compatibility

Compatibility in software refers to the ability of the software to work without integration errors with other versions of itself.
As there are multiple possible versions of the SDK, there are different types of compatibility.
In all of the examples bellow we will refer to the same V1 and V2 createPost function.

```ts
// V1
function createPost(title: string, content: string): Promise<Post> { /* EMPTY */ }
// V2
function createPost(title: string, content: string, subtitle?: string): Promise<Post>  { /* EMPTY */ }
```

## Backward Compatibility

Clients with  the new version V2 can use the previous version V1 interface.
Our createPost V2 interface is backwards compatible because it can work seamlessly with code written for the V1 version.

Analogy: USB 3.0 devices are expected to work with USB 2.0 ports

Time Traveling Analogy: Backwards compatibility in SDKs is like a time machine that allows you to take your future technology (represented by the new SDK) and use it in the past (represented by the older SDK) without issues.

## Forward Compatibility

Clients with old version V1 can use the new V2 version interface.
Forward compatibility ensures that the code can evolve along with the environment it operates in.

It might be confusing but the createPost V1 interface is NOT forwards compatible with respect to V2. 
A client with V1 SDK cannot use V2 interface, i.e. if I install V1 SDK in my project I cannot use the subtitle parameter.

Analogy: We expect USB 2.0 devices to work with USB 3.0 ports. Although they won't take advantage of USB 3.0 enhanced functionality.

Time Traveling Analogy: It's like traveling into the future, where newer technologies and capabilities exist, while  interacting with them seamlessly, even though your current tools or systems are not as advanced.

## Full Compatibility

With full compatibility, we have the best of both worlds. Users of V2 version can use V1 version, and users of V1 version can use V2 version.
Most of the time we talk bout backward compatibility, as when we have newer versions of something we naturally expect it to work with previous versions. 
If we shipped the V2 function as V1, we would have fully compatible software. But its easier said than done.

Analogy: We expect USB 2.0 devices to work with USB 3.0 ports as well as USB 3.0 devices are expected to work with USB 2.0 ports

Time Traveling Analogy: Full compatibility in SDKs is like to having access to a time machine that allows you to travel to any point in time, interact with any technology or environment, and return to your own time without any difficulties.

## Summary

We explored the realm of SDKs and their applications. We delved into the significance of a good DX. Additionally, we examined various compatibility types, including backward, forward, and full compatibility, accompanied by practical TypeScript examples.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. 

I trust that it proved valuable!
