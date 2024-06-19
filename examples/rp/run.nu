
def main [] {
  let INFO = $"\n(ansi green_bold)INFO:"

  # read src/bin 
  echo $'($INFO) Please choose which binary to build(ansi reset)'
  let src_bin_list = (ls ([. src bin *rs] | path join)).name
  # echo $'DBG: src_bin_list = ($src_bin_list)'
  let select_list = (
    $src_bin_list | each {
      |x|
      $x | (parse -r '(?P<noext>\w+)\.rs').noext.0
    }
  )
  echo 'Select which bin to build'
  let name = ($select_list | input list -f)

  echo $'($INFO) Building ($name) ...(ansi reset)'
  cargo run --bin $name --release
}


