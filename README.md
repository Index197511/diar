# diar

A directory favorite tool in Rust.

## Usage

- Add a favorite directory named `foo` to diar:

  - The current directory:

    `diar add foo`

  - Specify the full path of a directory:
    
    `diar add -p /path/to/directory foo`

- Show the list of added to diar:

  `diar list`

- Delete `foo` from diar
  
  `diar delete foo`

- Jump to `foo` that added directory path to diar:

  `diar-jump foo`

  Don't forget the `-`, please wait for the future ;(

- For more options refer to help:

  `diar -h`

## Installation

### Linux

- Install cargo

  - `curl -sSf https://static.rust-lang.org/rustup.sh | sh`

- Add the following to your `.bashrc`

  ```bash
  diar-jump(){
    local selected=$(diar jump $1)
    local flag=0
    if [[ -n $selected ]]; then
      if [ $(echo $selected | grep -e "Is this what you are jumping?") ]; then
        diar jump $1
        flag=1
      fi
      fi [[ $1 = "-h" ]]; then
        diar jump $1
      fi
      if [[ $flag -ne 1 ]]; then
        \cd $selected
      fi
    fi
  }
  ```

- Install the diar

  - `cargo install diar`
