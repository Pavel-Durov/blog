# Cross Platform CLI application with GO and Cobra

## Abstract

This article will cover the process and the involved components of building a [Command-Line Interface (CLI)](https://en.wikipedia.org/wiki/Command-line_interface)[1] application using the GO programming language. We will cover the required libraries, directory structure, configuration files, testing, and Cross-Platform build process.

## Command-line interpreter (CLI)

**CLI** stands for [Command-Line Interface](https://en.wikipedia.org/wiki/Command-line_interface) [1]. **CLI** application receive input from the user, do some computational tasks and produces an output. Compared to a [**Graphical User Interface (GUI)**](https://en.wikipedia.org/wiki/Graphical_user_interface) [2], **CLI** applications require fewer system resources since interactions with it don't involve graphics.

One of the cool things (at least in my opinion) about **CLI** applications is that when designed with composition in mind and having a well-defined In/Out interface. These applications can be composed together (similar to function composition) as one solution 🤓. 

For example, if we have two **CLI** application `A` and `B`, we can create a composed solution of `A.B` having input of `A` and output of `B`. 

## Cross-Platform

Cross-Platform[5] applications are designed to work on more than one computing platform. For example, we can build the same software but run it on Linux, Windows and Android devices. These applications are also referred to as multi-platform, platform-agnostic or platform-independent.

The idea is very cool. Cause who wants to maintain more software than needed? But it also needs to be considered in the software design. Cross-Platform applications need to consider any Operatic System (OS) specifics. We will see examples of it when dealing with the local filesystem in the sections below 👇.

## **CLI** project setup

We're going to use [Cobra](https://cobra.dev/) [3] **CLI** framework. **Cobra** is a very powerful, extendable and delightful framework to work with. You won't regret it, I promise 😶!

Go version used:
```shell
$ go version
go version go1.19 darwin/arm64
```

Initialise our project:

```shell
$ go mod init github.com/Pavel-Durov/cli-demo
go: creating new go.mod: module github.com/Pavel-Durov/cli-demo
```

Install **Cobra** and **CobraCLI**.  **CobraCLI** will create our application and add **CLI** commands.

```shell
$ go get -u github.com/spf13/cobra/cobra
$ go install github.com/spf13/cobra-cli@latest
```

Now we can use **CobraCLI** to initialise our **CLI** app:

```shell
$ cobra-cli init
```

That's it. We have a working app! It should have the following structure:

```shell
$ tree 
├── cmd
│   └── root.go
├── go.mod
├── go.sum
└── main.go
```

We have the `main.go` file, which is the main entry point of our application. And one single **CLI** command called `root` that is the main entry point of **Cobra** framework. It's also a general convention to have application entry points for GO projects in the `cmd` directory.

Run our brand-new **CLI** app:

```shell
$ go run ./main.go 
A longer description that spans multiple lines and likely contains
examples and usage of using your application. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.
Nothing match to see yet; we get the default message. Let's add some functionality. 
```

For this demo, we're going to build a calculator **CLI** application. I know, very exciting!

Adding our first command

```go
//file: ./cmd/add.go
package cmd

import (
	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add operator",
	Long:  `Add operator, adds two integers and prints the result.`,
	Run: func(cmd *cobra.Command, args []string) {
		num1, _ := cmd.Flags().GetInt32("n1")
		num2, _ := cmd.Flags().GetInt32("n2")
		cmd.Printf("%d + %d = %d\n", num1, num2, num1+num2)
	},
}

func init() {
	addCmd.Flags().Int32("n1", 0, "--n1 1")
	addCmd.Flags().Int32("n2", 0, "--n1 2")
	addCmd.MarkFlagRequired("n1")
	addCmd.MarkFlagRequired("n2")
}
```

We could use **CobraCLI** for that as well. But I decided to go manual here.

Note that we defined the `Use` property, which means that in order to use `add` command, we need to specify first `add` in our **CLI** parameters.

Wire **CLI** commands together.
```go
//file: ./cmd/root.go
package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "[command]",
	Short: "A CLI calculator",
	Long:  `A CLI calculator that can add and subtract two numbers.`,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(addCmd) // adding add command to root
}
```

Since our commands are part of the same package called `cmd` - the import and configuration are very straightforward.

Rerun our command, this time with `-h` flag:

```shell
$ go run ./main.go  -h
A CLI calculator that can add and subtractwo numbers.

Usage:
  calc [command]

Available Commands:
  add         Add operator
  completion  Generate the autocompletion script for the specified shell
  help        Help about any command

Flags:
  -h, --help   help for calc

Use "calc [command] --help" for more information about a command.
```

As you can see, cobra did a lot of work for us. It did configure how to parse flags, help messages etc.

Run the actual command with specified parameters:

```shell
$ go run ./main.go  add --n1=1 --n2=3
1 + 3 = 4
```
We have a fully-fledged **CLI** application that can add two numbers and print the result to the `stdout`.

Let's add another command! This time we will add substitution.

It will be as simple as:
```go
//file: ./cmd/sub.go
package cmd

import (
	"github.com/spf13/cobra"
)

var subCmd = &cobra.Command{
	Use:   "sub",
	Short: "Sub operator",
	Long:  `Sub operator, subtracts two integers and prints the result.`,
	Run: func(cmd *cobra.Command, args []string) {
		num1, _ := cmd.Flags().GetInt32("n1")
		num2, _ := cmd.Flags().GetInt32("n2")
		cmd.Printf("%d - %d = %d\n", num1, num2, num1-num2)
	},
}

func init() {
	subCmd.Flags().Int32("n1", 0, "--n1 1")
	subCmd.Flags().Int32("n2", 0, "--n1 2")
	subCmd.MarkFlagRequired("n1")
	subCmd.MarkFlagRequired("n2")
}
```

Same as before, add the new command to the `root`:
```shell
...
rootCmd.AddCommand(subCmd)
...
```


Give it a go:
```shell
$ go run ./main.go  sub --n1=10 --n2=4
10 - 4 = 6
```

It works exactly as intended! You can imagine how using the same process we can extend our **CLI** application further.


## Adding some tests

I like tests, and I think you should too 😎. Adding unit tests in GO is very straightforward; however, testing **CLI** commands like Cobra might be a bit tricky. That's why I wanted to demonstrate how to do it.

Adding test to root **CLI** command:
```go
// file: cmd/root_test.go
func TestTypeLocal(t *testing.T) {
	buf := new(bytes.Buffer)
	rootCmd.SetOut(buf)
	rootCmd.SetArgs([]string{"sub", "--n1=10", "--n2=4"})

	err := rootCmd.Execute()
	if err != nil {
		fmt.Println(err)
	}
	if buf.String() != "10 - 4 = 6\n" {
		t.Errorf("Expected 10 - 4 = 6, got %s", buf.String())
	}
}
```
Here, we set a buffer as an out stream for the cobra command and pass **CLI** arguments (aka flags), then we assert the output result - nothing fancy.

Run the tests:

```shell
$ go test ./...
ok      github.com/Pavel-Durov/cli-demo/cmd     0.207s    
```

## Adding profile/settings

What do we do if we want to store application configuration across sessions, or maybe we want to have secrets such as API keys defined outside of our application code. Whatever the reason, cobra got your back! Actually [Viper](https://github.com/spf13/viper)[4] got your back. **Viper** is a configuration management tool for Go applications. **Viper** and **Cobra** works great together.

Install ***Viper***:
```shell
$ go get github.com/spf13/viper
```
Configure Viper in our init function, root cmd:

```go
// file: cmd/root.go
func initConfig() {
    home, err := os.UserHomeDir()
	cobra.CheckErr(err)
	viper.AddConfigPath(home)
	viper.SetConfigType("yaml")
	viper.SetConfigName(".calc")
	viper.ReadInConfig()
}

func init() {
	cobra.OnInitialize(initConfig)
    ...
}
```
If we create a local **YAML** file called `.calc` in our `$HOME` directory (cause that's what we configured) with the content:

```shell
$ cat ~/.calc
username: kimchi
```

We can now read these values in our application:

```shell
username := viper.Get("username")
if username != nil {
    fmt.Println("Hello", username)
}
```

We don't have to use **YAML** or the `$HOME` directory, this setup can be configured in multiple ways.

### Note on `$HOME` directory

Notice how we used `os.UserHomeDir()` to get the user's home directory. This is important if we want to build a [**Cross-Platform**](https://en.wikipedia.org/wiki/Cross-platform_software)[6] application. We could've hardcoded the path to the file. But why should we? GO has great platform-agnostic library support - `os.UserHomeDir()` will return the path to the `$HOME` directory specific to the machine it's running on without us changing a single line of code!

👉 On Unix (including macOS), it returns the `$HOME` environment variable

👉 On Windows, it returns `%USERPROFILE%`

👉 On Plan 9, it returns the `$HOME` environment variable

## Building our **CLI** application

Go has an incredible build system that comes with everything we need. 

We can easily build our application for multiple architectures and operating systems (OS):

### Linux target build
```shell
$ CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -o out/linux-arm64-calc -ldflags="-extldflags=-static" # linux, arm64 arch
$ CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o out/linux-amd64-calc -ldflags="-extldflags=-static" # linux, amd64 arch
```

### Mac (aka darwin) target build
```
$ CGO_ENABLED=0 GOOS=darwin GOARCH=arm64 go build -o out/darwin-arm64-calc -ldflags="-extldflags=-static" # mac, arm64 arch
$ CGO_ENABLED=0 GOOS=darwin GOARCH=amd64 go build -o out/darwin-amd64-calc -ldflags="-extldflags=-static" # mac, amd64 arch
```

### Windows target build
```shell
$ CGO_ENABLED=0 GOOS=windows GOARCH=arm64 go build -o out/windows-arm64-calc -ldflags="-extldflags=-static" # windows, arm64 arch
$ CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -o out/windows-amd64-calc -ldflags="-extldflags=-static" # windows, amd64 arch
```

if we run all these build commands, we'll get these binaries:

```shell
$ ls -l ./out/
-rwxr-xr-x  1 ... darwin-amd64-calc
-rwxr-xr-x  1 ... darwin-arm64-calc
-rwxr-xr-x  1 ... linux-amd64-calc
-rwxr-xr-x  1 ... linux-arm64-calc
-rwxr-xr-x  1 ... windows-amd64-calc
-rwxr-xr-x  1 ... windows-arm64-calc
```
### Environment Variables
#### GOOS

You probably noticed that the only thing that changed between the build targets is the GOOS environment variable. And that's all you need to change with go-build tooling! It is seriously that easy to use!

#### GOARCH

This is where we specify the **CPU** architecture we're targeting.

See all the supported `GOOS` and `GOARCH` combos:

```shell
$ go tool dist list
aix/ppc64
android/386
android/amd64
android/arm
android/arm64
darwin/amd64
....The list goes on
```

#### CGO_ENABLED

We also used `CGO_ENABLED` environment variable. `CGO_ENABLED=1` leads to faster and smaller builds - it allows ti dynamically load the host OS's native libraries. However, it relies on a host OS, a dependency we would like to avoid! Otherwise, if our code relies in the host libraries, our code behaviour might differ from machine to machine.

### Linker flags - ldflags

We used flags called `ldflags` - `ld` stands for [linker](https://pkg.go.dev/cmd/link) [6] therefore `ldflags` stands for linker flags. A linker is a program that "links" the pieces of the compiled source code into the binary outcome. We are passing `extldflags` to our linker. According to the link tool documentation, these flags are passed to the external linker. Long story short, we're using these flags to indicate to the GO build tool to include all the dependencies into the binary and not rely on them provided by the environment it's running in. We set the flag as `-static`, indicating that the binary should include all its dependencies. If not specified, our binary would be dynamically linked. For the same reasons as with `CGO_ENABLED`, we would like to avoid it here.

## Summary

We've seen how to setup **Cobra**-based **CLI** applications from scratch. We touched on **Cross-Platform** application properties, such as platform-agnostic filesystem paths. We added tests and briefly overviewed Linker and the GO build tooling with different target configurations. 

Building GO applications for multiple targets is straightforward and fun once you get the basics.

This write-up was for my own sake of understanding and organising my thoughts as it was about knowledge sharing. I hope it was helpful. If you have questions/objections/observations/complaints, don't hesitate to reach out!

Full source code can be found 👉 [here](https://github.com/Pavel-Durov/blog/tree/main/src/cross-platform-cli-apps-with-go-and-cobra/examples).

## References

[1] https://en.wikipedia.org/wiki/Command-line_interface

[2] https://en.wikipedia.org/wiki/Graphical_user_interface

[3] https://cobra.dev/

[4] https://github.com/spf13/viper

[5] https://en.wikipedia.org/wiki/Cross-platform_software

[6] https://pkg.go.dev/cmd/link
