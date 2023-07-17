## Benchmarking with hyperfine

## Abstract

In this article, we will cover basic concepts of software benchmarking and its challenges as well as practical applications using Hyperfine benchmarking tool.

## Benchmarking

A benchmark is a program that is used to evaluate another program's performance. It is an act of running a program or a set of programs, to assess its relative performance. 

Usually, benchmarks are run using multiple tests and some kinds of average calculation to reduce the effect of anomalies (aka outliers). By anomaly, we mean anything that is unexpected that affects the measurement result.


## Benchmarking Challenges

On the surface, it sounds easy. Let's say we have two programs P1 and P2 and we want to compare their runtime performance. 
Why not just time their execution and compare the results?

The most naive approach would be doing something like:

```shell
$ time P1
real    0m1.005s
user    0m0.002s
sys     0m0.004s

$ time P2
real    0m2.005s
user    0m0.004s
sys     0m0.002s
```
Looking at the real-time reported by the `time` command we might conclude that P1 is twice as faster as P2.

But we might be measuring the wrong thing! 

Our software runs in a process, that runs on the operating system that runs on hardware. 

Each one of these components might have its optimizations when it comes to running programs. 

Measuring the wrong thing will lead us to the wrong conclusions. So we need to be careful about what we measure.

## Benchmark metrics

To do program benchmarking, we need first to identify what metrics we care about. 
The most common metric would be time, but in some applications, we might care more about other things like memory for example.

## Things to consider 

### Measurement Impact

It happened to me personally, one time I decided to measure web application performance using console.log. Soon after I realize that it was not my program that was really slow, it was the console.log functions that slow it down significantly.
When measuring performance we need to be careful with the impact of the measurements on the program we measure.

### Warmup

As we mentioned, we might have optimizations that will be included in our program execution - so we need to make sure we don't measure the difference between optimized and non-optimized versions.

The easiest way to address that is by doing a warmup. Wormup is when you run the target program before measuring performance to prepare the environment so that we can have consistent measurements when benchmarking. 

For example, in programs that have intensive disk usage, the results will be influenced by disk cache. 

Fun fact:
Warmups are also a thing when it comes to distributed applications - AWS Lambda for example. They have "Cold" and "Warm" starts.

### Machine state

When we run benchmarks, we need a stable, consistent environment. If for example, we run benchmarks on a machine that has other processes running, these processes might affect the executions since they might consume machine resources like CPU, memory, disk etc...
We need a quiet machine, ideally something like a server that has minimal processes running.

### Preparation and Cleanup

Sometimes we need to make sure that we do cleanup and preparation before and after we execute benchmarking so that following runs won't be affected by previous executions.
We can run preparation before and cleanup after benchmarks to ensure we have a clean environment.

### Run in isolation

Same concerns as with warmups, we want to avoid runtime/compiler optimizations so we would like to spawn each benchmark test in a separate, isolated process. 

### Average Multiple Runs

The general technique of handling outliers when it comes to measuring a sequence of data points is to calculate the mean value of the results.
Instead of analyzing a single run, we analyse the mean value of multiple runs.

## How to benchmark

There's no single best way to do it. We can write a custom tool that would support process spawning, warmups, multiple iteration and result comparison. Or we can use tools that are already made for that purpose.

Let's talk about hyperfine! It's easy to install, easy to use and overall just do what it needs to do when it comes to the benchmarking.

For demonstration purposes, I am going to pick up one of the benchmarks from the [Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html) project.

Let's pick up [Python Fasta](https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/fasta-python3-2.html), because why not?

Running it as is, should look something like that:

```shell
python ./fasta.py 10
>ONE Homo sapiens alu
GGCCGGGCGCGGTGGCTCAC
>TWO IUB ambiguity codes
cttBtatcatatgctaKggNcataaaSatg
>THREE Homo sapiens frequency
taaatcttgtgcttcgttagaagtctcgactacgtgtagcctagtgtttg
```
Now, let's measure it with hyperfine.

```shell
$ hyperfine 'python ./fasta.py 10'
Benchmark 1: python ./fasta.py 10
  Time (mean ± σ):      14.9 ms ±   0.7 ms    [User: 12.5 ms, System: 2.3 ms]
  Range (min … max):    14.0 ms …  21.3 ms    187 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interference from other programs. It might help to use the '--warmup' or '--prepare' options.
```

## Hyperfine - warmup
Notice that Hyperfine gave us a warning because I'm running this benchmark on my DIRTY machine. Ideally, I would run it on some kind of isolated server when I have minimal interference from other processes but sometimes you need to work with what you have.

Let's add the warmup process to our benchmarking:

```shell
$ hyperfine --warmup 5 python ./fasta.py 10' 
Benchmark 1: python ./fasta.py 10
  Time (mean ± σ):      15.2 ms ±   0.6 ms    [User: 12.7 ms, System: 2.5 ms]
  Range (min … max):    14.0 ms …  18.3 ms    194 runs
```

And the warning is gone! Still, for accurate results - run it on a quiet machine.

## Hyperfine - comparing multiple programs

To compare multiple programs we just add more commands as hyperfine arguments. We can also export results as a markdown:

```shell
$ hyperfine --warmup 5 --runs 10 'python3 ./fasta.py 10'  'python3 ./fasta.py 1' --export-markdown md
```

Result:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `python3 ./fasta.py 10` | 17.9 ± 1.5 | 16.2 | 34.0 | 1.00 |
| `python3 ./fasta.py 1` | 18.0 ± 1.1 | 16.5 | 25.2 | 1.01 ± 0.10 |

## Hyperfine Prepare and Cleanup

Sometimes one might need to run a command prior to the benchmarks. That can include cleanup, configraion or anything really that can come in a form of a shell command.

Adding prepare command to our benchmarking.
For example, deleting some kind of local cache file:

```shell
$ hyperfine --prepare 'rm /tmp/test.cache' --warmup 5 'python ./fasta.py 10'  'python ./fasta.py 1'
```

Similar thing with cleanup:

```shell
$ hyperfine --cleanup 'rm /tmp/output.txt' --warmup 5 'python ./fasta.py 10'  'python ./fasta.py 1'
```

## hyperfine execution stages

Overall the hyperfine execution can be illustrated as:
```
warmup -> setup -> prepare -> command 1 -> prepare -> command 2 -> cleanup
```

## Hyperfine ouptut

Let's analyze the data we got from hyperfine:

```shell
$ hyperfine --warmup 5 python ./fasta.py 10' 
Benchmark 1: python ./fasta.py 10
  Time (mean ± σ):      15.2 ms ±   0.6 ms    [User: 12.7 ms, System: 2.5 ms]
  Range (min … max):    14.0 ms …  18.3 ms    194 runs
```

`min .. max` - minimum and maximum running times
`User` - time spent in user space, amount of time spent on the program itself
`System` - time spent in system space, amount of time spent on OS functions related to the program
`runs` - total count of benchmark runs
`mean` - is the average execution time of the benchmarks
`± σ` - is the standard deviation, representing the amount of variation from the mean
`

# Summary

We covered the basics of benchmarking, the challenges and things that need to be considered when running benchmarking.
We also covered how to address these issues with the Hyperfine tool.
More information about Hyperfine can be found here
https://github.com/sharkdp/hyperfine
This writing was for my own sake of understanding and the organization of my thoughts as it was about knowledge sharing.
I hope it was helpful. If you have questions/objections/observations/complaints, don’t hesitate to reach out!
