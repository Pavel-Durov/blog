# macOS FS vs Linux FS - case sensitivity

# Abstract

In this article, I will overview the case-sensitivity difference between macOS default filesystem (FS) and Linux filesystem with some practical coding examples and gotchas.

# Intro
Unix and Linux might sound the same to some of us, but they are different beasts. 
macOS aka OS X or Max OS X is a **Unix-based** operating system, while Linux is **Unix-like** OS. There are many articles explaining the difference [1]. No need to repeat. 

# Experiment - maxOS FS case sensitivity

Let's do an experiment 🤓. 
Assuming that you're running on macOS, create a file called `entrypoint.sh` with content:
```
#!/bin/bash

echo "hello"
```
Try running it as:
```shell
$ bash ./entrypoint.sh
hello
```

Now rename it to `Entrypoint.sh` (with capital E) and run again:
```shell
$ bash ./entrypoint.sh
hello
```
Works anyway! File names are case insensitive, meaning that you can run it even as:
```shell
$ bash ./eNtryPOInt.sh
hello
```

I don't know how about you, but when I found it for the first time I was really surprised. 
When working with a mac, on the surface, it looks and feels very Linuxy, the terminal, the shell, the commands, the paths... It's all been a lie 😭.

## Let's see how case-sensitivity would work on Linux using Docker
Define a simple dockerfile:
```docker
FROM alpine:3.14
WORKDIR /poc
COPY ./Entrypoint.sh /poc/Entrypoint.sh
ENTRYPOINT ["/bin/sh", "/poc/entrypoint.sh" ]
```
Here we're copying the `entrypoint.sh` file and running it as the `ENTRYPOINT` command. Notice that we run the lowercased file but we copied in uppercased file!
Well, running it won't work since Linux filesystem is case-sensitive:
```shell
$ docker build -t macOS-FS . && docker run -t macOS-FS
/bin/sh: can't open '/poc/entrypoint.sh': No such file or directory
```
If we change the entrypoint to `/poc/Entrypoint.sh` it should work. Try it yourself!


## FS case-sensitivity and Git

Another thing to note is that since FS is case-insensitive, these changes will not be tracked in Git 😶. I know! Quite shocking to stop thinking of Git as the source of the absolute truth of text-based file changes! The same experiment can be done by renaming one of the tracked files and seeing the Git-detected changes 🤯. It got me personally multiple times over the past years when programs worked locally but failed to launch in Linux-based containers or in my lovely CI/CD pipelines.

## Using macOS case-sensitive file system
It's possible to have a case-sensitive filesystem on macOS. For that, we need to set the disc to use APFS (Case-sensitive) format:
"APFS (Case-sensitive): Uses the APFS format, is case-sensitive to file and folder names. For example, folders named “Homework” and “HOMEWORK” are two different folders." [2].

However, it might be risky. Most macOS applications don’t recognize a case-sensitive file system. And probably won’t work as expected. So wouldn't recommend it. On the other hand, it might make sense to set a separate partition on your drive to be case-sensitive, especially if you work with Linux-based systems quite often. But not the whole drive.

# Conclusion
macOS and Linux might look and feel the same, but in reality, they are quite different. Filesystem case sensitivity is only one example of such diffrence.
I hope that this short overview of case-sensitivity differences was useful. And you won't be caught off-guard next time your application won't run when Git didn't pick up on file rename changes on macOS.

# References

[1] https://www.softwaretestinghelp.com/unix-vs-linux/

[2] https://support.apple.com/en-gb/guide/disk-utility/dsku19ed921c/mac