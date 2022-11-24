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
package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "calc",
	Short: "A CLI calculator",
	Long:  `A CLI calculator that can add and subtractwo numbers.`,
}

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add operator",
	Long:  `Add operator, adds two integers and returns the result.`,
	Run: func(cmd *cobra.Command, args []string) {
		num1, _ := cmd.Flags().GetInt32("n1")
		num2, _ := cmd.Flags().GetInt32("n2")
		result := num1 + num2
		cmd.Printf("%d + %d = %d\n", num1, num2, result)
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(addCmd)
	addCmd.Flags().Int32("n1", 0, "--n1 1")
	addCmd.Flags().Int32("n2", 0, "--n1 2")
	addCmd.MarkFlagRequired("n1")
	addCmd.MarkFlagRequired("n2")
}
```
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


# References
[1] https://cobra.dev/