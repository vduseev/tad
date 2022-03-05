# Requirements


## Functional requirements

| ID | BR | S | Category | Description |
| -- | -- | ------ | -------- | ----------- |
| <a name="fr-1"></a>FR-1 | <a href="#br-1">BR-1</a> |  | Execution | Execute an executable file |
| <a name="fr-2"></a>FR-2 | <a href="#br-1">BR-1</a> |  | Execution | Execute executable file in Docker |
| <a name="fr-3"></a>FR-3 | <a href="#br-1">BR-1</a> |  | Discovery | Find test case pair using exact names of files |
| <a name="fr-4"></a>FR-4 | <a href="#br-1">BR-1</a> |  | Discovery | Find test case pair using its name |
| <a name="fr-5"></a>FR-5 | <a href="#br-1">BR-1</a> |  | Discovery | Find all standard test cases in current directory
| <a name="fr-?"></a>FR-? | <a href="#br-2">BR-2</a> |  | Analysis  | Compare execution results with test case's output file |


- [ ] `FR-2` Run the executable in an isolated container.
- [ ] `FR-3` Run the isolated container in the cloud with a predictable performance.
- [ ] `FR-9` Run executables asynchronously being able to stop/pause them at any moment.
- [ ] `FR-10` Send the source code to HackerRank and get back the results.

### Executables
- find executable in specified dir
- find executable in specified dir by reading "executable" param .fate/config file in that dir
- find executable by direct provided path
- find all executables in specified directory tree

### Test cases
- find test cases in current directory

## Inputs and outputs

### Inputs

- [ ] `IO-1` Current directory is considered to be the current execution context.
- [ ] `IO-2` Expected directory layout to support `fate`:

  ```shell
  repository-with-solutions/    # your repository, where all solutions are contained
    algorithms/                 # subsection of solutions to specific domain
      solve-me-first/           # single problem
        input/                  # directory with input files
          input00.txt           # input file of pair "00"
          input01.txt
          ...
        output/                 # directory with output files
          output00.txt          # output file matching the "00" input file
          output01.txt
          ...
        python3/                # solutions in specific language (see language naming below)
          solution01.py         # one of the solutions
          mega-test-case.in     # a custom test case input test file thrown in during development
          meta.out              # a corresponding test case output file for quick test
          bottom-up/            # directory with another solution
            bottom-up.py        # another solution
          ...
        java8/
        cpp/
        ...
    data-structures/
    sql/
    ...
  ```

  ```shell
  repository-with-solutions/    # your repository, where all solutions are contained
    algorithms/                 # subsection of solutions to specific domain
      solve-me-first/           # single problem
        X/                  # directory with input files
          input00.txt           # input file of pair "00"
          input01.txt
          ...
        output/                 # directory with output files
          output00.txt          # output file matching the "00" input file
          output01.txt
          ...
        python3/                # solutions in specific language (see language naming below)
          solution01.py         # one of the solutions
          mega-test-case.in     # a custom test case input test file thrown in during development
          meta.out              # a corresponding test case output file for quick test
          bottom-up/            # directory with another solution
            bottom-up.py        # another solution
          ...
        java8/
        cpp/
        ...
    data-structures/
    sql/
    ...
  ```

  Another layout

  ```shell
  testing/                      # your working directory
    solve.sh                    # solution in bash
    tc0.in                      # pair of input output test case files with same name but different extension
    tc0.out
    input-basic.txt             # pair of input output file with different names but same extension
    output-basic.txt
    input/                      # input output files sorted into directories
      input00.txt
      input34.txt
    output/
      output00.txt
      output34.txt
    special_test_cases/
      input01.txt
      output01.txt
      input02.txt
      output02.txt
      tc1.in
      tc2.out
  ```

- [ ] `IO-3` Input/output files are searched in the following order:
  - [ ] `IO-3.1` Matching pairs of files in the current directory with a naming convention being `input*.txt` and `output*.txt`.
  - [ ] `IO-3.2` Matching pairs of files with the same pattern but being placed in the `./input/` and `./output/` subdirectories of the current directory.
  - [ ] `IO-3.3` Same two attempts but in the parent directory `./..`.
  - [ ] `IO-3.4` Again, same two attempts but in the grandparent directory `./../..`
  - This means that `fate` traverses 3 levels of directories: the current directory and into two levels above the current directory.
  - Higher level directories are traversed if the current directory had no matching pairs of input/output files.
  - [ ] `IO-4` Read input and output interactively from stdin.
- [ ] `IO-5` The executable solution is searched in the following order:
  - [ ] `IO-5.1` `fate [path]` specified as the absolute/relative path in the first positional argument during invokation
  - [ ] `IO-5.2` First executable file in the current directory `./`.
  - [ ] `IO-5.3` First executable in any of the subdirectories `./*/`.
  - [ ] `IO-5.4` First executable in any of the sub-subdirectories `./*/*/`
- [ ] `IO-6` Language codes for solution languages are as following:

  | Code     | Language                  |
  | -------- | ------------------------- |
  | bash     | Bash ~>4                  |
  | c        | C98                       |
  | csharp   | C#, .Net 5.0              |
  | cpp      | C++98                     |
  | cpp14    | C++14                     |
  | go       | Go                        |
  | java7    | Java 7                    |
  | java8    | Java 8                    |
  | js       | JavaScript (Node.js)      |
  | pypy2    | PyPy 2                    |
  | pypy3    | PyPy 3                    |
  | python2  | Python 2.7.15             |
  | python3  | Python 3.8                |
  | postgres | PostgreSQL 11.0           |
  | oracle   | Oracle 11.0               |

### Outputs

#### Exceptions and error handling

- [ ] `ERR-1` If no mathing input/output pairs are found then:
  - [ ] `Matching input/output files are not found` message is printed to stderr.
  - [ ] `fate` exits with code `1`.
- [ ] `ERR-2` If no executable is found then:
  - [ ] `Executable solution not found` message is printed to stderr.
  - [ ] `fate` exits with code `2`.
- [ ] `ERR-3` If executable does not exist:
  - [ ] `Executable <path> does not exist` message is printed to stderr.
  - [ ] `fate` exits with code `3`.
- [ ] `ERR-4` If specified executable file has no execution rights:
  - [ ] `File <path> cannot be executed` message is printed to stderr.
  - [ ] `fate` exits with code `4`.

#### Warnings

- [ ] `WAR-1` If input/output file size is larger than 1GiB:
  - [ ] `File <path> is larger than 1GiB` message is printed to stdout.
