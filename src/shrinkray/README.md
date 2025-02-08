## Shrinkeay


https://github.com/DRMacIver/shrinkray


## Installation

```shell
pipx install git+https://github.com/DRMacIver/shrinkray.git
```

# Buggy example

```shell
 $ python3 ./factorial.py 3
Traceback (most recent call last):
  File "/Users/pd/git-repos/Pavel-Durov/blog/src/shrinkray/src/./factorial.py", line 11, in <module>
    print(factorial(arg))
          ^^^^^^^^^^^^^^
  File "/Users/pd/git-repos/Pavel-Durov/blog/src/shrinkray/src/./factorial.py", line 7, in factorial
    return n * factorial(n - 2)
               ^^^^^^^^^^^^^^^^
  File "/Users/pd/git-repos/Pavel-Durov/blog/src/shrinkray/src/./factorial.py", line 7, in factorial
    return n * factorial(n - 2)
               ^^^^^^^^^^^^^^^^
  File "/Users/pd/git-repos/Pavel-Durov/blog/src/shrinkray/src/./factorial.py", line 7, in factorial
    return n * factorial(n - 2)
               ^^^^^^^^^^^^^^^^
  [Previous line repeated 996 more times]
RecursionError: maximum recursion depth exceeded
```

# Shrinkray

```shell
shrinkray my-test-case is_interesting.sh
```

# Test the intrestingness of the test case

```shell
bash ./is_interesting.sh ./factorial.py 3 || echo $?
1
```

```shell
bash ./is_interesting.sh ./factorial.py 2 && echo $?
0
```

```
chmod u+x ./is_interesting.sh
```

exec
```
$ shrinkray  --timeout 10 --parallelism 50 --no-clang-delta  is_interesting.sh ./factorial.py
Reduction completed!
Deleted 145 Bytes out of 168 Bytes (86.31% reduction) in 17.26 seconds
```

Where my-test-case is some file you want to reduce and is_interesting.sh can be any executable that exits with 0 when a test case passed to it is interesting and non-zero otherwise.

Variant test cases are passed to the interestingness test both on STDIN and as a file name passed as an argument. Additionally for creduce compatibility, the file has the same base name as the original test case and is in the current working directory the script is run with. This behaviour can be customised with the --input-type argument


Previously used other tools like CReduce which are fine for C-baed