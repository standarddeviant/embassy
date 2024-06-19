
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

  let bpath = [. target thumbv7em-none-eabi release $name] | path join
  let hpath = [. $'($name).hex'] | path join
  let upath = [. $'($name).uf2'] | path join

  echo $'($INFO) Building ($name) ...(ansi reset)'
  cargo build --bin $name --release

  echo $'($INFO)  converting bin-to-hex ...(ansi reset)'
  arm-none-eabi-objcopy -O ihex $bpath $hpath

  echo $'($INFO) converting merged .hex to merged .uf2 file ...(ansi reset)'
  python uf2conv.py -f 0xADA52840 -c -b 0x1000 -o $upath $hpath
}


