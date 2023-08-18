# Developer Experience and SDKs


## Introduction


## What's SDK
An SDK (Software Development Kit) is a collection of software components bundled as a package. 
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


While this principle may appear elementry, it's essential. 
A tool should precisely do what it claims to do. Surprisingly, numerous tools have an inclination to do things that a developer would not reasonably anticipate.

Consider this scenario: you've incorporated an SDK to seamlessly interface with a Restful API. Yet, upon implementation in your project, it unexpectedly generates hefty files on your disk due to an unanticipated optimisation process.

### Comprehensive documentation

Documentation does not need not be verbose, it should be precise. Crafting clear documentation is one of the most challenging parts in software engineering. 
It is imperative for documentation to remain up-to-date, striking a balance between brevity and comprehensiveness.

### Ease of use

Overall it needs to be intuitive. A developer can look at the code and can understand what to do with it without deep dive into the documentation. If it's language-specific then it needs to follow the language standards and not reinvent the wheel. It should look familiar and friendly.

### Ease of use

It should be intuitive. A developer should glance at the code and immediately get how to work with it without the need for extensive documentation exploration. 

When tailored to a specific programming language, it should faithfully stick to the language's conventions and avoid unnecessary deviations. The code's appearance should invoke familiarity and approachability.

The end-to-end use of the tool should be easy. That includes installation, configuration and actual use.

### Adaptable

It should be designed to be flexible. That includes modularity, configuration options and version management.

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

## Dx with node.js SDKs

Now, let's talk about a specific SDK implementation in TypeScript.

In order to deliver a good DX, we need first to understand the client. What TypeScript engineers expect from the SDK.

Usually:
- Promises and Async/Await
- Easy to use SDK
- Installed with one of the goto package managers like `npm``
- It should have functional code examples - copy, paste, execute.
- Type Definitions - TypeScript is statically-typed language, types are the basic components.
- Type Safety - SDKs should enforce type safety throughout their interfaces.
- Modules Support - compatibility with modern module systems like CommonJS and ES6 modules
- Optional parameters - Optional parameters enhance flexibility and usability.


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


Consider our SDK `createPost`` function:
```ts
function createPost(title: string, content: string): Post {
  //....
}
```

Let's say we want to allow multiple ways of creating Post objects.
And the obvious tool of choice for this job is, you guessed it right - optional parameters.

```ts
function createPost(title: string, content: string, subtitle?: string){
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

Intuitively I would expect the title to be the first function argument, followed by the subtitle and then the content.
But we can't just change the order at will, we will be breaking V1 version where suddenly all the content will be set as subtitles - that is unacceptable.


And what will happen when we add another parameter to our function?

TODO: next

```js
function newPost(title: string, content?: string, callback?: string, date?: Date){
  //....
}
```
Now this function can be used as:

```js
newPost("My Title")
newPost("My Title", "My Content")
newPost("My Title", "My Content", (data) => console.log(data));
newPost("My Title", "My Content", (data) => console.log(data), new Date());
newPost("My Title", undefined, (data) => console.log(data), new Date());
newPost("My Title", undefined, undefined, new Date());
// etc...
```
I'm not going to list all the permutations here, but I think the idea is clear.

What would be better?
We can use an object for that!

```js
interface Params {
  title: string;
  content?: string;
  callback?: function;
  date?: Date;
}

function newPost(params: Params) : Promise<number>{
  return Promise.resolve(0);
}

```
and with js:
```js
/**
 * @function newPost
 * @param {Object} params Post creation params
 * @param {string} params.title Post title
 * @param {string} params.content Post content
 * @param {function} params.callback Post update callback
 * @param {Date} params.date Post creation date
 * @returns {Promise<number>} Created Post ID
 */
function newPost(params){
  return Promise.resolve(0);
}
```

# Compatibility

## Backward

Consumers of the new version (V+1) can use previous version (V).

Analogy: 

We expect USB 3.0 devices to work with USB 2.0 ports


HDMI 10.0 was designed in a way so that clients (TVs) would be able to connect to old 9.0 producers (streaming devices).

## Forward
Consumers of old versions (V) can use version (V+1).

Analogy:
We expect USB 2.0 devices to work with USB 3.0 ports

Forward compatibility is hard. Supporting it means that the design was built with future revolution in mind.

## Full

It's hard, but its doable!
With full compatibility, we have the best of both worlds. Users of V+1 version can use V version, and users of V version can use V+1 version.

Most of the time we talk bout backward compatibility, as when we have newer versions of something we exepect it to work with previous versions. But we also expect older versions to work with newer, so most of the time we actually speaking of full compatibility.


## Other things to consider:
Semantic Versioning (SemVer), Changelog, LTS (Long-Term Support)
## Summary

We need Node.js SDKs to be intuitive

Simple
Intuitive
Easy to use and clear documentation
Needs to be Adaptive
Be forward and backwards compatible

