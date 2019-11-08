# diar

A directory favorite tool in Rust.

## Usage

- Add a favorite directory named `foo` to diar:

  - The current directory:

    `diar add foo`

  - Specify the full path of a directory:
    
    `diar add -p /path/to/directory foo`

- Rename a favorites from `foo` to `bar`:

  - `diar rename foo bar`

- Delete `bar` from diar:
  
  `diar delete bar`

- Jump to `foo` that added directory path to diar:

  `diar-jump foo`

  Or, you'd like to jump to the current project root directory:

  `diar-jump -p`

  Don't forget the `-`, please wait for the future ;(

- Show the list of added to diar:

  `diar list`

- For more options refer to help:

  `diar -h`

## Installation

### Linux

- Install cargo

  - `curl -sSf https://static.rust-lang.org/rustup.sh | sh`

- Add the following to your `$HOME.bashrc` or others

  ```bash
  diar-jump(){
    local result=$(diar jump $1)
    if [ -n "$result" ]; then
      if echo "$result" | grep -e "^Error:" > /dev/null || [ "$1" = "-h" ]; then
        echo -e "$result"
      else
        \cd $result
      fi
    fi
  }
  ```

- Install diar

  - `cargo install diar`
