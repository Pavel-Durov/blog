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
