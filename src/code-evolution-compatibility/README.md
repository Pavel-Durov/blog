
# Abstract
TODO

# Introduction
TODO

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

# Sammary
TODO