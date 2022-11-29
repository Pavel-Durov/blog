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
