# Building GO Cli app

## Abstract

We will cover the process and the involved components of building a CLI application using Go programming languages. We will cover the libraries we use, directory structure, configuration files and project structure. 


## What are CLI applications


## Start the project

I am using go version 1.19:

```shell
$ go version
go version go1.19 darwin/arm64
```

Initialise our project:

```shell
λ go mod init github.com/Pavel-Durov/cli-demo
go: creating new go.mod: module github.com/Pavel-Durov/cli-demo
```
And install cobra [1]. Cobra is a CLI framework for Go, very powerful and extendable.

```
$ go get -u github.com/spf13/cobra/cobra
$ go install github.com/spf13/cobra-cli@latest
``
Now we can use cobra-cli to initialise our applicaton:
```shell
$ cobra-cli init
```


That's it, we have a working app! Should look something like that:
```shell
$ tree 
.
├── cmd
│   └── root.go
├── go.mod
├── go.sum
└── main.go
```

Give it a go!

```shell
$ go run ./main.go 
A longer description that spans multiple lines and likely contains
examples and usage of using your application. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.

```

Nothing match to see yet. Let's add parameters.
We're going to build a calculator CLI application. I know, very exciting!

Let's add our first command - addition:
```go
//file: ./cmd/add.go
package cmd

import (
	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add operator",
	Long:  `Add operator, adds two integers and returns the result.`,
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
And let's hook it into our root command:

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
	Long:  `A CLI calculator that can add and subtractwo numbers.`,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(addCmd)
}
```

Since our commands are all part of the same package - the import and configuration are very straightforward.

run help command:
```
λ go run ./main.go  -h
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
run the actual command:
```go
λ go run ./main.go  add --n1=1 --n2=3
1 + 3 = 4
```

Let's add another command! This time we add substitution.

It will be as simple as that:
```go
//file: ./cmd/sub.go
package cmd

import (
	"github.com/spf13/cobra"
)

var subCmd = &cobra.Command{
	Use:   "sub",
	Short: "Sub operator",
	Long:  `Sub operator, subtracts two integers and returns the result.`,
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
And configure in root command, as we did with `add` command:
```go
...
rootCmd.AddCommand(subCmd)
...
```

Give it a go:
```shell
$ go run ./main.go  sub --n1=10 --n2=4
10 - 4 = 6
```

Works exactly as intended!
You can imagine that we can extend our CLI application

## Adding some tests
I like tests and I think you should to 😎. Adding unit tests in go is very straightforward, however testing CLI commands like cobra might be a bit tricky, hence I wanted to demonstrate how to do it.

Adding test to root CLI command:
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
We set a buffer as an out stream for the cobra command and pass CLI arguments (aka flags), then we assert the output result - nothing fancy.

## Adding profile/settings

TODO:

## Building our CLI application

Go has an amazing build system that comes with everything you need.
Let's build our application for multiple architectures and operating systems (OS):

```shell
# linux
$ CGO_ENABLED=0 GOOS=linux GOARCH=arm64 go build -o out/linux-arm64-calc -ldflags="-extldflags=-static" # linux, arm64 arch
$ CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o out/linux-amd64-calc -ldflags="-extldflags=-static" # linux, amd64 arch
# CGO_ENABLED=0 mac
$ CGO_ENABLED=0 GOOS=darwin GOARCH=arm64 go build -o out/darwin-arm64-calc -ldflags="-extldflags=-static" # mac, arm64 arch
$ CGO_ENABLED=0 GOOS=darwin GOARCH=amd64 go build -o out/darwin-amd64-calc -ldflags="-extldflags=-static" # mac, amd64 arch
# CGO_ENABLED=0 windows
$ CGO_ENABLED=0 GOOS=windows GOARCH=arm64 go build -o out/windows-arm64-calc -ldflags="-extldflags=-static" # windows, arm64 arch
$ CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -o out/windows-amd64-calc -ldflags="-extldflags=-static" # windows, amd64 arch
```
Note that we used `ldflags`, `ld` stands for `linker`, `ldflags` stands for linker flags.
The [linker](https://pkg.go.dev/cmd/link) is the program that "links" together the pieces of the compiled source code into the binary outcome. 
We also specify `extldflags`. According to the link tool documentation, these flags are passed to the external linker.
Long story short, we're using these flags to indicate to go build-tool to include all the dependencies into the binary. 

We also used `CGO_ENABLED` environment variable. CGO_ENABLED=1 leads to faster and smaller builds - it can dynamically load the host OS's native libraries. However, it relies on a host OS, and that's a dependency that we would like to avoid!

Get all supported targets just for fun:
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

That's it :)

# Summary

We've seen how to setup cobra based CLI applications from scratch. We added tests and we touched Linker and the build tooling with different modes that Go support. Building GO applications for multiple targets is very fun and straightforward once you get the basics.

I hope that was helpful and that you will build your own CLI application soon enough!


# References

[1] https://cobra.dev/

[2] https://pkg.go.dev/cmd/link