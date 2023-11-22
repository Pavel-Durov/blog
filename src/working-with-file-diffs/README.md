## File Comparison - A Practical Guide to Working with Diff and Patch Files"

## Abstract

This article explores file comparison techniques through the practical utilization of diff and patch CLI commands. 
It will cover the difference between context and unified format and illustrate how to generate and apply patch files locally.


## Introduction

Let's say we have two text files, file1.txt and file2.txt and we want to compare them. 

```python 
def main():
    for i in range(10):
        print("Hello, World!")


if __name__ == "__main__":
    main()

```

```python
import sys


def main():
    for i in range(int(sys.argv[1])):
        print("Hello, World!")


if __name__ == "__main__":
    main()

```
## Generating diff files

Of course, one of the ways to compare file content is just by eyeballing both files. But this approach is not error-prune, and after doing it for a while, we get tired and make more mistakes.


We can use the `diff` command to see the difference between both files. 

Let's check the `diff`  manual first:
```shell
$ man diff
NAME
     diff – differential file and directory comparator
```

Let's run it:
```shell
$ diff file1.py file2.py 
```

Which should produce this output:
```diff
1,3d0
< import sys
< 
< 
5c2
<     for i in range(int(sys.argv[1])):
---
>     for i in range(10):
```

that's useful but we can do better:

```shell
diff -u -L "RIGHT" -L "WRONG"  file1.py file2.py 
```
output:
```diff
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

Here, we gave our file labels using `-L` arguments and set it to be unified diff.

We could also use context diff using `-c` instead of `-u`, let's talk about that birefly.

## Conext diff vs Unified diff

Context diff and Unified diff are two formats of diff command outpur that represent the differences between two files. 
Each has its advantages in different scenarios.


### Context Diff 


Looks something like that:
```shell
diff  -c -L "RIGHT" -L "WRONG"  file1.py file2.py 
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
This format provides a more extensive context for the difference.
Each change block begins with *** and --- and it shows several lines of unchanged content around each change.

### Unified Diff

```shell
diff  -u -L "RIGHT" -L "WRONG"  file1.py file2.py 
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
This format is more compact compared to the Context diff.
It shows a unified difference. Each block changes stars with @@.

### Applying diff

One of the cool features of diff is that the diff can be easily applied to a file.
Let's see how to do so.

First, we're going to generate a diff file:

```shell
$ diff  -u -L "RIGHT" -L "WRONG"  file1.py file2.py  > mydiff.diff
```

File content:

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

Let's apply this diff to one of the files.

Let's check the `patch` manual first:
```shell
$ man patch
NAME
    patch – apply a diff file to an original
```

For that we're going to use `patch` command:

```shell
$ patch -b file1.py < ./mydiff.diff 
patching file 'file1.py'
```
And that's it, we have the changes from `file2.py` applied to `file1.py`.
Since we specified `-b` the patch also created a backup file with the original content.

Both files should look something like this:

```shell
$ cat file1.py
def main():
    for i in range(10):
        print("Hello, World!")


if __name__ == "__main__":
    main()
```

```shell
$ cat file1.py.orig 
import sys


def main():
    for i in range(int(sys.argv[1])):
        print("Hello, World!")


if __name__ == "__main__":
    main()

```
I found this especially useful when I'm debugging program outputs locally and I need to keep track of and compare different output versions over time.
Or when I need to share quickly my local changes with someone else.
Of course, we can use `git` to track history, but sometimes working with diff files directly is faster and more straightforward.
We can also apply these diff files with git using `git apply`.


## Summary

In this article we explored file comparison through the practical utilization of diff and patch files.

We eplained the diffrence fo context and unified diff format and provided practical instruction on how to apply patch files.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.

I trust that it proved valuable!