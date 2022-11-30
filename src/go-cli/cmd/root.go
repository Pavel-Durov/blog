package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use:   "[command]",
	Short: "A CLI calculator",
	Long:  `A CLI calculator that can add and subtractwo numbers.`,
	Run: func(cmd *cobra.Command, args []string) {
		username := viper.Get("username")
		if username != nil {
			fmt.Println("Hello", username)
		}
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

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
	rootCmd.AddCommand(addCmd)
	rootCmd.AddCommand(subCmd)
}
