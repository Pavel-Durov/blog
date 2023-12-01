## File Comparison and Patching in Unix Environments and Git

## Abstract

In this article, we will explore file comparison techniques through the practical utilization of diff and patch Unix command-line utilities. 
We will cover how to compare files using context and unified formats and illustrate how to generate and apply patch files as well as show how can we work with patch files in git repositories.

## Introduction

Let's say we have two text files, `file1.py` and `file2.py` and we want to compare them. 
It doesn't matter what is the content of the files, as long as they have different content.

Content of `file1.py`:

```python 
def main():
    for i in range(10):
        print("Hello, World!")


if __name__ == "__main__":
    main()

```

Content of `file2.py`:

```python
import sys


def main():
    for i in range(int(sys.argv[1])):
        print("Hello, World!")


if __name__ == "__main__":
    main()

```
One way of comparing files is through a manual process, you put them side by side and eyeball the changes. But this approach is error-prune. 

Let's see how can we use command line tools to help us with this task.

## Diff and Patch files

The `diff` command can be used to get the difference between both files. This difference, when saved as a file is called patch file.


Let's see it in action:

```shell
$ diff file1.py file2.py 
1,3d0
< import sys
< 
< 
5c2
<     for i in range(int(sys.argv[1])):
---
>     for i in range(10):
```

That default format of the diff output is a context format.
Without going into too much details, let's talk about diff formats.

## Conext vs Unified format

Context diff and Unified diff are two formats of diff command output. It represents the changes in a human-readable format, making it easier to understand the difference. 

Without going into too much detail, let's compare these formats.

### Context Diff 

```shell
$ diff  -c -L "RIGHT" -L "WRONG"  file1.py file2.py 
*** RIGHT
--- WRONG
***************
*** 1,8 ****
- import sys
- 
- 
  def main():
!     for i in range(int(sys.argv[1])):
          print("Hello, World!")
  
  
--- 1,5 ----
  def main():
!     for i in range(10):
          print("Hello, World!")
```
Here, we gave our file labels using the `-L` argument and set it to use context format using the `-c` flag (that's also the default option).
This format provides an extensive context of the difference as it shows several lines of unchanged content around each change.

### Unified Diff


```shell
$ diff -u -L "RIGHT" -L "WRONG"  file1.py file2.py 
--- RIGHT
+++ WRONG
@@ -1,8 +1,5 @@
-import sys
-
-
 def main():
-    for i in range(int(sys.argv[1])):
+    for i in range(10):
         print("Hello, World!")
```

Here, we gave our file labels using `-L` argument and set it to be a unified format using the `-u` flag.
This format is way more compact when compared to the context format, it shows a unified difference in a concise way, with no context.
We can work with either format, choose whatever you like. 
I am going to use a unified format in the following sections, just cause I am used to it.

### Applying patch files

One of the cool things that we can do with diff and patch files, is that the patch files can be easily applied to a file.
That way we can save changes with `diff` and then select what we want when we want it and just apply it!

That might seem like a no-brainer, but the first time I saw it in action I was really impressed. It's simple, yet very powerful.

First, we're going to generate a patch file with diff command:

```shell
$ diff  -u -L "RIGHT" -L "WRONG"  file1.py file2.py  > mydiff.diff
```

Patch file `mydiff.diff` content:

```shell
$ cat mydiff.diff
--- RIGHT
+++ WRONG
@@ -1,8 +1,5 @@
-import sys
-
-
 def main():
-    for i in range(int(sys.argv[1])):
+    for i in range(10):
         print("Hello, World!")
```

Next, we're going to apply it to one of the files using `patch` command.

```shell
$ patch -b file1.py < ./mydiff.diff 
patching file 'file1.py'
```
And that's it, we have the changes from `file2.py` applied to `file1.py`.
Since we specified `-b` flag the `patch` also created a backup file with the original content.

If you check file1.py it should be identical to `file2.py` since we applied the diff from `file2.py` to `file1.py`.

I found this especially useful when I'm debugging program outputs locally and I need to keep track and compare different versions of the oputput over time. Or when I need to share quickly my local changes with someone else.

Of course, we can use git to track file history, but sometimes working with diff and patch files directly can be more straightforward.

## Using Patches in Git

Here we're going to illustrate one of the ways of working with patch files in Git. This is by far not an extensive guide but it should give a general feeling of how it works.

We're going to create a patch file from one branch and apply it to another branch using built-in git commands.

Let's create a new project and init git repo:
```shell
$ mkdir git-patch-example  && cd git-patch-example && git init
$ echo "init change" > file.txt # make some changes
$ git add file.txt && git commit -m "init commit" # add file and commit
```

Create a side branch with some changes:
```shell
$ git checkout -b branchA # create new branch
$ echo "another change" > file.txt # make some changes
$ git add ./patchMe.txt && git commit -m "test commit" # add file and commit
```

Generate a patch file:
```shell
$ git format-patch main -o patches/ # Generate a patch. It will be stored in a "patches" directory.
patches/0001-test-commit.patch
```
Let's see the patch file content:
```shell
$ cat patches/0001-test-commit.patch

---
 file.txt | 1 -
 1 file changed, 1 deletion(-)

diff --git a/file.txt b/file.txt
index 17819c8..e69de29 100644
--- a/file.txt
+++ b/file.txt
@@ -1 +0,0 @@
-init change
--
```
That should look familiar!

Let's create yet another branch from `main` and apply the patch:

```shell
$ git checkout main && git checkout -b branchB
$ git apply patches/0001-test-commit.patch # apply the patch
```
That should apply the changes from the patch file to the branch.

I hope it's obvious these patch files can be useful when comparing changes.
In the real-world, these patch files can also be shared between developers or applied changes across different git repositories.

## Summary

In this article, we explored file comparison through the practical utilization of `diff` and `patch` commands.

We explained the difference between context and unified formats and provided practical examples of how to work with patch files in Unix based environment and when working with Git repositories.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.

I trust that it proved valuable!