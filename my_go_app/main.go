package main

/*

#cgo CFLAGS: -I./include
#cgo LDFLAGS: -L./include -lmylib_c_static -ldl
#include "mylib_native.h"
*/
import "C"
import "fmt"

func main() {
	fmt.Println("Hello Go")
	C.mylib_call()
}
