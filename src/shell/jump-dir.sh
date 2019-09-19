diar-jump(){
  local selected=$(diar jump $1)
  local flag=0
  if [[ -n $selected ]]; then
    if [ $(echo $selected | grep -e "Is this what you are jumping?") ]; then
      diar jump $1
      flag=1
    fi
    if [[ $flag -ne 1 ]]; then
      \cd $selected
    fi
  fi
}
