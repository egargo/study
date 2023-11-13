(* https://www.codewars.com/kata/55a70521798b14d4750000a4/train/ocaml *)

let greet(name: string): string =
    "Hello, " ^ name ^ " how are you doing today?"

let main() =
    print_string(greet "bee");;

main()
