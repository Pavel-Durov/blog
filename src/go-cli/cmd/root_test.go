package cmd

import (
	"bytes"
	"fmt"
	"testing"
)

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
