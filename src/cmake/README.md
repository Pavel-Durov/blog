# C++ "Hello, World!" with Make and CMake

## Abstract

In this short article, we're going to cover Make and Cmake tools on a very high level. We will illustrate its applications with a simple C++ "hello world" program.

Some basic knowledge of C++ is required, but even if you're completely new to it, I think it should be clear. 

When I first had to work with Cmake, although I knew already how to work with `Makefiles`, it was really confusing. The intention here is to give an overview of these tools without going into too much detail. 

## Introduction

Makefiles are configuration files used by the Make automation tool (aka build-system). `Makefiles` are set of specifications defining how to perform an action related to the project such as building, testing, linting etc.
Make is widely used, especially in Unix-based operating systems. It's a convenient tool, for project organisation. It can also be integrated with development environments and language-specific features.Make configuration files are named `Makefile` by convention.

CMake is not a build-system. Cmake generates another system's build files, Makefiles in our case. It supports directory hierarchies and applications that depend on multiple libraries. Cmake configuration files are named `CMakeLists.txt` by convention. Cmake is widely used in C/C++ projects.

## "Hello, World!" using C++ 

Let's create a simple "hello world" application in C++:

```cpp
// file: hello_world.cpp
#include <iostream>
    
int main()
{
    std::cout << "Hello, World!" << std::endl;
}
```

In order to compile we'll use `clang++` (the C++ compiler):
```
$ clang++ hello_world.cpp -o hello_world
```

This should result in a binary artifact `hello_world`.

Let's run it:
```
$ hello_world 
Hello, World!
```

## "Hello, World!" using Make

Now, let's add this build command to our `Makefile`

```makefile
# file: Makefile
default:
	clang++ hello_world.cpp -o hello_world.o
```

And run it using make (make sure we're in the same directory as the `Makefile`):

```shell
$ make
```
That should run the default Make command that would do the compilation for us. We moved the actual shell command to our Makefile. We don't need to remember how to use `clang++` anymore, we can just run `make` command to build our application, for better or for worse.

## "Hello, World!" using Cmake

Whatever you want to do in your project, Cmake probably can do that.

In order to run `cmake`, we need to define a few things first. 
> The way I'm going to configure it is just a suggestion, diffrent projects will have diffrent setup.

Let's create a file called `CMakeLists.txt` with the following content:

```cmake
# file: CMakeLists.txt
cmake_minimum_required(VERSION 3.22.0)
project(hello_world)
add_executable(${PROJECT_NAME} hello_world.cpp)
```
This Cmake configuration is self-explanatory.
We defined the minimum version of Cmake we're willing to use. 
Then we set the project name and configure our executable defined in `hello_world.cpp` file.

Next, we're going to build our project.

```shell
$ cmake -S . -B ./build
```
Here, we're telling Cmake where to find our source code (identified by `-S`) and where to build our project (identified by `-B`).

The build directory should look something like that:
```shell
$ ls -l  ./build

-rw-rw-r-- 1 CMakeCache.txt
drwxrwxr-x 5 CMakeFiles
-rw-rw-r-- 1 cmake_install.cmake
-rw-rw-r-- 1 Makefile
```
This auto-generated `Makefile` will have a bunch of things added by Cmake.

So far, the only thing we did was to generate the `Makefile`.

Now, Let's run it!

Navigate into the build directory:

```shell
$ cd ./build
```
And run `make`:

```shell
$ make 
[ 50%] Building CXX object CMakeFiles/hello_world.dir/hello_world.cpp.o
[100%] Linking CXX executable hello_world
[100%] Built target hello_world
```

That should build our `hello_world.cpp` binary based on our specification in `CMakeLists.txt`.
We can run it:
```shell
$ ./hello_world 
Hello, World!
```

Yey! It worked!

That's about as minimal as you can get with Cmake.

Cmake has way more configurations, it includes features like Variables, Levels and Library linking. That will make your project management easier, especially when dealing with C/C++ code base.

### Linking Libraries

Let's say now, we want to use an external library in our C++ application.
The only thing we need is to add some configuration to CMakeLists.txt and library linkage to the target appication will be taken care of by the Cmake.

```makefile
// file: CMakeLists.txt
find_package(nlohmann_json 3.2.0 REQUIRED)
target_link_libraries(hello_world PRIVATE nlohmann_json::nlohmann_json)
```
If you setup the library as something that Cmake can find (usually its quite straightforward) then you all good and you can you it in your code:
```cpp
// file: hello_world.cpp
#include <iostream>
#include <nlohmann/json.hpp>

int main()
{
    std::cout << nlohmann::json::parse("{ \"msg\": \"Hello, World!\" }") << std::endl;
    return 0;
}
```
As before, build and run it:
```shell
$ cmake -S . -B ./build 
$ cd build
$ make
$ ./hello_world 
{"msg":"Hello, World!"}
```

# Sammary

We briefly covered how to use Make and Cmake tools, as well as demonstrated a simple "hello world" C++ application compilation and library linkage configurations. 

This writing was for my own sake of understanding and the organization of my thoughts as it was about knowledge sharing. I hope it was helpful. If you have questions/objections/observations/complaints, don't hesitate to reach out!

