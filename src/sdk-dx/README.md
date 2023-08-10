# Developer Experience and SDKs


## Introduction


## What's DX (Developer Experience)

It should be:

- Intuitive - someone can look at the code and can understand what todo without deep dive into documentation
- Easy to use - 
- and Adaptable (extendable) - tailor experience to the use


## Why should you have good DX?
If we have an easy to use software it will be naturally adopted, if it feels nice people will naturally talk about it

## Dx with node.js

First we need to understand the client. Node.js engineers will want:

- like async calls
- can the sdk just do it for me?
- provide with easy examples - compu, paste, execute - quickstart
- love optional parameters
- SDK should not do anything that it doesnt support to do!

## Example

Let's say we have the following API

```
POST /posts - Add new post
PUT /posts/{id}/alerts - Subscribe for post notifications
```

We would like to have something like that:

```js
import { Post } import 'posts-sdk';

const result = await Post.posts(new Post.Post('title', 'content'));
const result = await Post.alert("postId", "webhook url");
```

Things to consider:
How easy would it be to maintain the code I've developed using the SDK.

## Why we love optional parameters

Consider the following function
```js
function newPost(title: string, content: string, callback: string){
  //....
}
```

Let's say we our SDK to allow multiple usages of creating new Posts, sometimes with a callback and sometime without.

One way of accomplishing it, is by using optional parameters.
With optional parameters, we can use our function as following:

```js
function newPost(title: string, content?: string, callback?: string){
  //....
}

newPost("My Title")
newPost("My Title", "My Content")
// or 
newPost("My Title", "My Content", (data) => console.log(data));
```

Maybe that looks ok for now. But what will happen when we extend this function?
Let's say we add another optional parameter to it:

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


## Summary

We need Node.js SDKs to be intuitive

Simple
Intuitive
Easy to use and clear documentation
Needs to be Adaptive
Be forward and backwards compatible

