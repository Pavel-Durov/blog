# Developer Experience and SDKs
## Abstract.

This article overviews the realm of SDKs (Software Development Kit), discussing their benefits and core attributes. Through the exploration of TypeScript examples, we will also delve into the concepts of DX (Developer Experience) and various types of compatibility.


## Introduction
SDK provides a means of integration with external systems such as remote APIs, operating systems, devices or hardware platforms. 

By establishing consistent APIs and abstraction layers, SDKs simplify the integration process, providing a structured means to interact with underlying functionalities. SDKs facilitate the process of integration by providing consistent API (Application Programming Interface) and abstraction levels to the underlying functionality.

## What's SDK

An SDK is a collection of software components bundled as a package. 
This package encompasses everything necessary to effectively use the underlying system for which the SDK provides functionality.

Examples of SDKs:
Stripe SDK - https://stripe.com/docs/libraries
Adobe Acrobat SDK - https://opensource.adobe.com/dc-acrobat-sdk-docs/acrobatsdk/
Unity SDK - https://docs.unity.com/ads/en-us/manual/InstallingTheUnitySDK

## Advantages of utilizing SDKs

SDKs offer a streamlined approach to crafting applications for specific targets. They function as specialized toolkits. One of their key benefits lies in simplifying the integration process. This simplification is achieved by often hiding the complexities of internal implementation and providing an intuitive interface.

## SDKs vs APIs

Usually, an API (Application Programming Interface) is contained within an SDK. SDKs employ APIs in the background, enhancing them with additional functionality, comprehensive documentation, and practical examples.

Distinctively, SDKs are tailored to specific programming languages, while APIs maintain a higher level of abstraction. This distinction makes SDKs more user-friendly and readily adoptable due to their streamlined integration and developer experience.


## What's DX and why it's important

DX (Developer Experience) describes the interactions and feelings that a software developer has when working with a tool or piece of code. 
If you are familiar with the term UX (User Experience), then you can think of DX in the same terms where the user is the developer.

It might be subjective, but great DX is hard to deny. Generally speaking, when evaluating tools we should consider:

If we have an easy-to-use tool it will be naturally adopted by individuals or communities, if it feels good to people, they will naturally talk about it. Good DX improves productivity and the adoption tools.


### Explicit Functionality


While this principle may appear elementary, it's essential. 
A tool should precisely do what it claims to do. Surprisingly, numerous tools have an inclination to do things that a developer would not reasonably anticipate.

Consider this scenario: you've incorporated an SDK to seamlessly interface with a Restful API. Yet, upon implementation in your project, it unexpectedly generates hefty files on your disk due to an unanticipated optimisation process.

### Comprehensive documentation

Documentation does not need not be verbose, it should be precise. Crafting clear documentation is one of the most challenging parts in software engineering. 
It is imperative for documentation to remain up-to-date, striking a balance between brevity and comprehensiveness.

### Intuitive and easy to use

It should be intuitive. A developer should glance at the code and immediately get how to work with it without the need for extensive documentation exploration. 

When tailored to a specific programming language, it should faithfully stick to the language's conventions and avoid unnecessary deviations. The code's appearance should invoke familiarity and approachability.

The end-to-end use of the tool should be easy. That includes installation, configuration and actual use.

### Adaptability

It should be designed to be flexible. That includes modularity, configuration options and version management.

### Compatibility

To achieve good DX software needs to be designed with compatibility in mind. The worse DX is when you upgrade your SDK version and suddenly you need to fix all the places that this SDK is used in your project. We will talk more extensively about compatibility types and examples later on. 

### Quickstarts and Samples

Compact, functional examples that provide a comprehensive glimpse of the tool's capabilities are priceless. They trigger those illuminating "AHA" moments when, upon running the provided sample, everything effortlessly falls into place.

One of the best quickstarts is node.js express - https://expressjs.com/en/starter/hello-world.html
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

## TypeSCript Node.js SDKs

Now, let's talk about TypeScript SDK specifics.

To deliver a good experience, we need first to understand the client. 
We need to ask - What do TypeScript engineers expect from the SDK?

To name a few of these expectations:
- Easy-to-use SDK
- Promises and Async/Await
- Installed with one of the goto package managers like `npm`
- It should have functional code examples - copy, paste, execute.
- Type Definitions - TypeScript is a statically-typed language, and types are the basic components.
- Type Safety - SDKs should enforce type safety throughout their interfaces.
- Modules Support - Compatibility with modern module systems like CommonJS and ES6 modules
- Optional parameters - Optional parameters enhance the flexibility and usability of the SDK.

We're going to address most of these points, with a focus on code evolution and optional parameters.

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

## The evolution of optional parameters

Let's talk about optional parameters and how they affect code evolution over time.


Consider our SDK `createPost` function:
```ts
function createPost(title: string, content: string): Promise<Post> {
  //....
}
```

Let's say we want to allow multiple ways of creating Post objects.
And the obvious tool of choice for this job is, you guessed it right - optional parameters.

```ts
function createPost(title: string, content: string, subtitle?: string): Promise<Post> {
  //....
}
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

Intuitively I would expect the title to be the first function argument, followed by the subtitle and then the content. But we can't just change the order at will, we will be breaking V1 compatibility. If we did it will mean that for V1 suddenly all the content will be set as subtitles - that is unacceptable.

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

Compatibility in software SDKs refers to the ability of the SDK to work seamlessly with other versions of that software without causing errors. 
There are different compatibilities.

In all of the examples bellow we will refer to the same  `createPost` function:
```ts
// V1
function createPost(title: string, content: string): Promise<Post> { /* EMPTY */ }
// V2
function createPost(title: string, content: string, subtitle?: string): Promise<Post>  { /* EMPTY */ }
```

## Backward Compatibility

Consumers of the new version V+1 can use the previous version V.

Analogy: 
USB 3.0 devices are expected to work with USB 2.0 ports

Our V2 is backward compatible as existing V1 code that calls with only two arguments will still work with the V2 function.

```ts
await createPost("My Title", "My Content", "Subtitle");
```

## Forward Compatibility

Consumers of old version V can use version V+1.
Forward compatibility ensures that the code can evolve along with the environment it operates in. 

Analogy:
We expect USB 2.0 devices to work with USB 3.0 ports

It might be confusing, but our function is actually NOT forward-compatible.
Old version V cannot use V2 features of adding subtitles as part of post-creation. To make it compatible, we should've shipped V1 with an optional subtitle parameter that would be used in the future. That's what makes it so difficult - thinking about possible future extensions.

## Full Compatibility

That's really hard, but it's doable! 

With full compatibility, we have the best of both worlds. Users of V+1 version can use V version, and users of V version can use V+1 version.

Most of the time we talk bout backward compatibility, as when we have newer versions of something we naturally expect it to work with previous versions. 
But we also expect older versions to work with newer ones, so most of the time we actually speaking of full compatibility.

## Summary

We explored the realm of SDKs and their applications. We delved into the significance of a good DX. Additionally, we examined various compatibility types, including backward, forward, and full compatibility, accompanied by practical TypeScript examples.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. 

I trust that it proved valuable!
