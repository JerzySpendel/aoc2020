package main
import (
	"io/ioutil"
	"fmt"
)

func main() {
	data, err := ioutil.ReadFile("input")
	if err != nil {
		fmt.Println(err)
	}
	
}
