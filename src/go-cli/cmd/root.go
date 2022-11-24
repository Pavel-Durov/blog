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
