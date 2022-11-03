## Stackless

Same as [Stackless Python](https://wiki.python.org/moin/StacklessPython).

- The size of the C stack no longer limits recursions, instead they are limited by the amount of available heap memory. But that's not the major point.

- Stackless Python allows you to run many small tasks called "tasklets" thread (sounds like go routines). Running in completly decoupled way, communicating over channels.
