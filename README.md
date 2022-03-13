# tad

Command line tool to test solutions to algorithms and data structures problems.

![Crates.io](https://img.shields.io/crates/v/tad)
![Crates.io](https://img.shields.io/crates/l/tad)

The **goal** of the project is to make it **as simple as possible** to develop and test solutions
for algorithmic problems from websites such as HackerRank, LeetCode, GeeksForGeeks, and others.

* `tad` takes a bunch of input and output files you give it and runs the source code of your solution against them
* If you have Docker installed, `tad` can use it to compile and run most of the languages
* `tad` can show you a line by line difference between your answer and the correct one
* `tad` can launch an interactive debugger to debug your solution, if supported by the language

# How it works

`tad` is a small binary written in Rust. It does not know how to run the solutions for any languages
or how to compile them but it relies on pre-built container images or an explicitly provided commands
for that.

When launched, `tad` finds and parses a configuration, looks up input and output files from test cases,
runs the given solution against these test cases using a specified way, and finally, shows where the
error is.

## Requirements

If you want to use preconfigured environments to run the solutions, you need to have Docker installed.
Other than that `tad` is a lightweight statically compiled binary that has no other dependencies.

# Usage

For a directory containing a solution to the problem and a pair of input and output files

```shell
solution.py
input01.txt
output01.txt
```

`tad` can be launched like this:

```shell
$ tad solution.py
[1/3] Collecting cases [100%]
[2/3] Executing [100%]
[3/3] Verifying results [100%]
01 : Pass!
```

## Specifying configuration options

`tad` knows that this solution file is written in Python3 because we've specified it
inside the comment in the file:

*Example code is taken from HackerRank*

```python
# tad: env=python3 inputs=input*.txt outputs=output*.txt
data = input()
result = "answer: " + data
f = open(os.environ['OUTPUT_PATH'], 'w')
f.write(str(result) + '\n')
```

When `env` parameter is specified, `tad` uses Docker to run this solution.
But `tad` can also rely purely on command line options to specify the same parameters.
Consider this:

```shell
$ tad solution.py --env python3 --inputs 'input*.txt' --outputs 'output*.txt'
```

This will achieve the same result without having to specify this parameters in the comment
line of the solution's source code.

A third way to specify these parameters is to put them in a config file called `tad.yaml`:

```yaml
env: python3
inputs:
  - input*.txt
outputs:
  - output*.txt
```

Again, having the `tad.yaml` file like the one above placed in the same directory will allow `tad`
to parse these paramters from the config file, rater than looking for them in the source code
or in the command line options.

Finally, tad can pick up the configuration options from the environment variables. Example below
works in any common Linux/MacOS shell:

```shell
$ TAD_ENV=python3 TAD_INPUTS=input*.txt TAD_OUTPUT=output*.txt tad solution.py
```

## Launch against specific test case

Just specify the name of the test case as a second positional argument:

```shell
$ tad solution.py 01
```

## Do I have to use Docker?

When the `env` parameter is specified, `tad` will attempt to compile and launch the solution
in the container. `tad` relies on Docker being instaleld on the system in order to do that.

Using containers to run and test solutions has multiple benefints

* You get out of the box suppot for lnaugages
* Ability to run solutions in interactive debug mode using the IDE of your choice
* Automatic limitation of memory and cpu for each container
* Predictable environment that matches the test environments of platforms like HackerRank 1:1

However, you might need to run the solution without Docker. If you want to do this, you can
explicitly specify the `command` required to run the solution. When you do that, `tad` ignores
any `env`, `debug` or even path to the solution. But `tad` will still run the supplied command
against the found test cases and verify the output.

```shell
$ tad -i='input*.txt' -o 'output*.txt' --command "python3 solution.py"
```

*Notice how we've shortened the syntax of options this time: Here `-o` is the same as `--output`
and that rule applies to all other commands as well. See `tad --help` for details.*

The shell command above will launch whatever is given to the `command` as it is.

This ability is also useful if you compile and run a solution in a language that does not have
an out of the box container implementation in `tad`.

**Note**: *If you'd like a language to be added and you can tell us how to compile and run it,
we'd really appreciate you openning an [issue](https://github.com/vduseev/tad/issues)
with a request for new language and instructions on running it.* ❤️

# Installation

You have multiple options when installing `tad`.

**If you have Rust installed**

```shell
cargo install tad
```

**Get the binary from the GitHub release**

# License

This project is licensed under the terms of the Apache-2.0 license.