# Test native libraries

(Run this on linux)

First, create the static library
```
bazel build //mylib_nativ/...
```

Then, copy the generated binary:

```
cp bazel-bin/mylib_native/libmylib_c_static.a my_go_app/include
```

You should then be able to
```
cd my_go_app
go build
./my_go_app

# Should print the following:
# Hello Go
# hello rust
# mylib_c version: 1.0
```

If you want to strip the symbols, you can run the following

```
cd my_go_app
strip --keep-symbol=mylib_call include/libmylib_c_static.a -o include/libmylib_c_static-stripped.a
ranlib include/libmylib_c_static-stripped.a
```

Then, in `main.go`, replace:

```
#cgo LDFLAGS: -L./include -lmylib_c_static -ldl
```

With

```
#cgo LDFLAGS: -L./include -lmylib_c_static-stripped -ldl
```

Finally, run `go build`.

