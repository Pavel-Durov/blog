# Python logging module quirks

## Introduction
Python comes with the default, ready-to-use logging module. It's kind of nice cause we don't have to install additional libraries to start logging in to our application. However, for someone coming from other language backgrounds, such as Javascript, TypeScript, Java, C#, and Go. The Python logger module might resemble familiar tools, but it has its specifics. In this article, I would like to cover some quirks that I found along the way while I was working recently on a python project.


## Read the documentation

Yeah, that would be a sensible thing to do. Make a cup of coffee, relax and read the logging module [documentation](https://docs.python.org/3/library/logging.html) [1], all the 13 sub-pages, of course 🙂. Unfortunately, not everyone has the time to do so. I usually skim through the documentation, but when I deal with multiple programming languages daily, it's hard to hold all the language and framework-specifics in my head. Anyway, enough about me and my saga with Python documentation; let's look at the logger module.

## Log levels
Why do we need logging levels? Good question. Because depending on how we run our application, we might want to have different types of logs. For example, if we're in the developing phase, we would like to see verbose logs of what exactly happens in the application. But when we're ready for production, we would probably want to limit our logs to only errors - otherwise, we will have a lot of logs, where most of them are probably going to be just noise. By noise, I mean they will not have useful information while simultaneously being very verbose and distracting. So we need to understand what kind of information we need in our logs. Having everything everywhere usually will not get us far.

The logger module comes with several [levels](https://docs.python.org/3/library/logging.html#logging-levels)[2]:

```
CRITICAL = 50
ERROR = 40
WARNING = 30
INFO = 20
DEBUG = 10
NOTSET = 0
```
I think the names are quite self-explanatory, but you can always go back to the original [docs](https://docs.python.org/3/library/logging.html#logging-levels).
Each level has a number assigned to it. The higher the number, the higher its priority. For example, if we set the log level to be `WARNING`, we will only get logs with a priority higher than `WARNING`, i.e. higher than 30. Similarly, if we set it to `NOTSET` level we will get all the logs.

Let's run an example logging python application:
```python
import logging

logging.debug('test')
logging.info('test')
logging.warning('test')
logging.error('test')
logging.critical('test')
```

Expected output:

```shell
WARNING:root:test
ERROR:root:test
CRITICAL:root:test
```
Where did my logs go? We have only `WARNING`, `ERROR`, and `CRITICAL` 🤔. As we mentioned above, it's all about the level of priority set.

Let's set the log level as `DEBUG`:

```python 
import logging

logging.basicConfig(level=logging.DEBUG)

logging.debug('test')
logging.info('test')
logging.warning('test')
logging.error('test')
logging.critical('test')
```
Run it, and observe the output:

```shell
$ python main.py 
DEBUG:root:test
INFO:root:test
WARNING:root:test
ERROR:root:test
CRITICAL:root:test
```
As expected, we get all the logging levels 🥳.

## Logging to files

Similarly to what we did above, we can log into a local file:
```python 
import logging

logging.basicConfig(filename='test.log', filemode='w')
logging.warning('test')
```
expected test.log file content:
```shell
WARNING:root:test
```
There're more configuration options [available](https://docs.python.org/3/library/logging.html#logging.basicConfig) [3]

## Formatting the output

When we have multiple components in the same application logging message, it's natural to want them to log in the same format; this way, we can apply the exact configuration for message content, such as process name, level, and module names across our application.

```shell
import logging

logging.basicConfig(format='%(process)d-%(levelname)s-%(message)s')
logging.error('test')
```
Same here. More formatting options in the [docs](https://docs.python.org/3/library/logging.html#formatter-objects) [4]


## Capturing stack traces

What happens when we have an error? What information are we expected to see? Usually, we would want a short description of what happened and a stack trace. Stack trace provides information about the execution path, how we got to that error, function names, calls and execution context.


But first, Let's see if we can guess would the following log be. We're going to raise a `ZeroDivisionError` exception intentionally:

```python
import logging

try:
  1 / 0
except Exception as e:
  logging.error(e)
```
Looks ok, right? Well, that would just log:
```bash
$ python ./main.py 
ERROR:root:division by zero
```
I don't know about you, but the first time I saw this log in production, I was pretty puzzled about it. If you made it that far I salute you 🫡. That's one of the quirks I was referring to in my intro.

We need to add the `exec_info=True` parameter or use `logging.exception` function to get the stack trace information.

```python
import logging

try:
  1 / 0
except Exception as e:
  logging.error('OOPS', exc_info=True)
  # OR: logging.exception('OOPS')
```

Expected output:
```shell
ERROR:root:OOPS
Traceback (most recent call last):
  File "...py", line 4, in <module>
    1 / 0
ZeroDivisionError: division by zero
```

Yep. We got our stack trace. Let's move on 🚀.

## Module level logs, namespaces, or however you want call them

So far, we have used the default logger, aka the root logger. 
That's all fine, but most Python applications consist of multiple modules logging altogether. 
In that case, we would probably want to differentiate which logs come from which modules. I mean, why not?


Let's see an example of such a [module level](https://docs.python.org/3/library/logging.html#module-level-functions) logs [5]:

```python
import logging

module_logger = logging.getLogger('module_logger')
module_logger.warning('module_logger log message')

logging.warning('module_logger log message') 
```
Expected output:
```shell
module_logger test
root test
```

We created a custom logger instance named `module_logger`. Different from the root logger, its name is not part of the output. But have no fear! We can add to our output via [formatting](https://docs.python.org/3/library/logging.html#formatter-objects) [4]:

```python
import logging

logging.basicConfig(format='%(name)s %(message)s',)
module_logger = logging.getLogger('module_logger')
module_logger.warning('test')
logging.warning('test') 
```

Observe the output:
```shell
module_logger log message
WARNING:root:module_logger log message
```

Kind of cool. I would say 😎.

## Handlers

Let's have a look now at log [handlers](https://docs.python.org/3/library/logging.handlers.html)[6]. Handlers are helpful when we want to(surprisingly) handle logs in multiple ways, for example, send the output to various destinations. Each logger might have multiple handlers.

Example:

```python
import logging

logger = logging.getLogger(__name__)

logger.addHandler(logging.StreamHandler())
logger.addHandler(logging.FileHandler('out.log'))

logger.warning('test')
logger.error('test')
```
Let's run it. We should see the message both in our terminal and in the local file named: `out.log`. Notice that the file will not be overridden when we re-run the program; content will be appended. One common use of logging handlers is sending logs over the network. For example, log aggregation services.


## Structured Logs
We covered briefly in previous sections the log formatting. That helps with log structure. But once we start configuring our logger, we might find ourselves writing and maintaining a boilerplate code. 
I found [jsonlogger](https://pypi.org/project/python-json-logger/)[7] very useful for JSON-structured logs. It is easy to configure, it just works. Once installed, here's an example of how to use it:
```python
import logging
from pythonjsonlogger import jsonlogger

root_logger = logging.getLogger()
logHandler = logging.StreamHandler()
logHandler.setFormatter(jsonlogger.JsonFormatter())

root_logger.addHandler(logHandler)

root_logger.setLevel(logging.DEBUG)

logger = logging.getLogger('test')
logger.info('test')
logger.warning('test')
logger.error('test')
```
Expected output:
```shell
{"message": "test"}
{"message": "test"}
{"message": "test"}
```

JSON-structured logs are useful when processing log files for analytics or BI (business intelligence). As well as for searching. Searching and correlating log messages is very valuable when developing and troubleshooting.

## Logs and Unix streams
In Unix-based systems, we have `stdin`, `stdout`, and `stderr` streams. Unix streams are like everything else in Unix, treated as files 🐧. What I mean is that we can read and write to streams as if we were handling files. Well, kind of.

Ok, cool. how is it relevant to this post? 
Let's run one of the logging applications from above, and check which stream is it using:

```python
import logging

logger = logging.getLogger(__name__)

logger.addHandler(logging.StreamHandler())
logger.addHandler(logging.FileHandler('out.log'))

logger.warning('test')
logger.error('test')
```
We can run and redirect each stream to a separate file.
```shell
$ python logs.py 1> stdout.log 2> stderr.log
```

Here, 1 stands for `stdout`, 2 for `stderr`. We redirect `stdout` stream to `stdout.log` file and `stderr` to `stderr.log`. More info about [redirecting output](https://www.gnu.org/software/bash/manual/bash.html#Redirecting-Output) [8].


If we check your `stderr.log` file, it has all the content. How come? Was it an actual error? No? Well, that's the default behavior of the Python `logging` module. I found it quite surprising 😶. It's problematic cause if our application sends logs to some kind of log aggregation service such as (logstash)[https://www.elastic.co/logstash/] or (datadog)[https://www.datadoghq.com/] our logs will be identified as errors when in reality they are not.

We can add `stream` parameter to point to `stdout`:
```python
logger.addHandler(logging.StreamHandler(stream=sys.stdout))
```
We should be able to locate the log content in `stdout.log` now. 


# Streaming to both stdout and stderr segregation
When our application is monitored by other services, it can be useful to have the segregation of error and other logs.

One way to achieve to define multiple handlers:
```python
import sys
import logging
from pythonjsonlogger import jsonlogger

class CustomJsonFormatter(jsonlogger.JsonFormatter):
    def add_fields(self, log_record, record, message_dict):
        super(CustomJsonFormatter, self).add_fields(log_record, record, message_dict)
        log_record["level"] = record.levelname
        log_record["name"] = record.name
        log_record["message"] = record.getMessage()
        if record.levelname == "ERROR" or record.levelname == "EXCEPTION":
            log_record["exception"] = record.exc_info
            log_record["stacktrace"] = record

formatter = CustomJsonFormatter("%(level)s %(name)s %(message)s")

stdoutHandler = logging.StreamHandler(stream=sys.stdout)
stdoutHandler.setLevel(logging.DEBUG)
stdoutHandler.setFormatter(formatter)

stderrHandler = logging.StreamHandler(stream=sys.stderr)
stderrHandler.setLevel(logging.ERROR)
stderrHandler.setFormatter(formatter)

root_logger = logging.getLogger()
root_logger.setLevel(logging.DEBUG)

root_logger.addHandler(stderrHandler)
root_logger.addHandler(stdoutHandler)

```
However, that results in duplication of logs; the ERROR logs will result in both `stderr` and `stdout` streams. That's far from ideal.


But! We can use builtin `Filter Objects`. Example:

```python
class StdoutFilter(logging.Filter):
    def filter(self, record):
        return record.levelno >= ROOT_LEVEL and record.levelno < logging.ERROR
```
Here we defined a class that inherits from `logging.Filter`. It will decide for each log record whether it should be handled by the handler or not.

```python
import sys
import logging
from pythonjsonlogger import jsonlogger
from requests import TooManyRedirects


class CustomJsonFormatter(jsonlogger.JsonFormatter):
    def add_fields(self, log_record, record, message_dict):
        super(CustomJsonFormatter, self).add_fields(
            log_record, record, message_dict)
        log_record["level"] = record.levelname
        log_record["name"] = record.name
        log_record["message"] = record.getMessage()
        if record.levelname == "ERROR" or record.levelname == "EXCEPTION":
            log_record["exception"] = record.exc_info
            log_record["stacktrace"] = record


ROOT_LEVEL = logging.DEBUG


class StdoutFilter(logging.Filter):
    def filter(self, record):
        return record.levelno >= ROOT_LEVEL and record.levelno < logging.ERROR


class StderrFilter(logging.Filter):
    def filter(self, record):
        return record.levelno >= logging.ERROR


formatter = CustomJsonFormatter("%(level)s %(name)s %(message)s")

stdoutHandler = logging.StreamHandler(stream=sys.stdout)
stdoutHandler.setFormatter(formatter)
stdoutHandler.addFilter(StdoutFilter())

stderrHandler = logging.StreamHandler(stream=sys.stderr)
stderrHandler.setFormatter(formatter)
stderrHandler.addFilter(StderrFilter())


root_logger = logging.getLogger()
root_logger.setLevel(ROOT_LEVEL)

root_logger.addHandler(stderrHandler)
root_logger.addHandler(stdoutHandler)


logging.debug('test')
logging.info('test')
logging.warning('test')
logging.error('test')
logging.critical('test')
```

> 

Run it:
```shell
$ python logs.py 1> stdout.log 2> stderr.log
```
And observe file content:

`stdout.log`:
```text
{"level": "DEBUG", "name": "root", "message": "test"}
{"level": "INFO", "name": "root", "message": "test"}
{"level": "WARNING", "name": "root", "message": "test"}

```

`stderr.log`:
```text
{"level": "ERROR", "name": "root", "message": "test", "exception": null, "stacktrace": "<LogRecord: root, 40, ...main.py, 53, \"test\">"}
{"level": "CRITICAL", "name": "root", "message": "test"}
```


Another promising library I've encountered is [loguru](https://loguru.readthedocs.io/en/stable/overview.html)9]. However, I couldn't figure out how to configure stream segregation there.

## Summary
Python logging module should be flexible enough to fit the general purpose. First, however, we need to understand how it works, especially if we come from another programming language background. Python logging module default behavior, and the difference between root and module loggers, were alien to me. But I survived 💪. I hope it makes sense and gives a high-level overview of how to use the logging module and its configurations.

Another thing to mention is that all these logging examples could've been implemented as [configuration files](https://docs.python.org/3.10/howto/logging-cookbook.html#custom-handling-of-levels)[10].

## References

[1] https://docs.python.org/3/library/logging.html

[2] https://docs.python.org/3/library/logging.html#logging-levels

[3] https://docs.python.org/3/library/logging.html#logging.basicConfig

[4] https://docs.python.org/3/library/logging.html#formatter-objects

[5] https://docs.python.org/3/library/logging.html#module-level-functions

[6] https://docs.python.org/3/library/logging.handlers.html

[7] https://pypi.org/project/python-json-logger/

[8] https://www.gnu.org/software/bash/manual/bash.html#Redirecting-Output

[9] https://loguru.readthedocs.io/en/stable/overview.html

[10] https://docs.python.org/3.10/howto/logging-cookbook.html#custom-handling-of-levels
