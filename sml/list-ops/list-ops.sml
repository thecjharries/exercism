fun concat (lists: int list list): int list =
    List.concat lists

fun reverse (list: int list): int list =
    List.rev list

fun filter (function: int -> bool, list: int list): int list =
    List.filter function list

fun map (function: int -> int, list: int list): int list =
    List.map function list

fun append (list1: int list, list2: int list): int list =
    list1 @ list2

fun length (ns: int list): int =
    List.length ns

fun foldl (function: int * int -> int, initial: int, list: int list): int =
    let
        fun foldl' _ i [] = i
          | foldl' f i (x::xs) = foldl' f (f (i, x)) xs
    in
        foldl' function initial list
    end

fun foldr (function: int * int -> int, initial: int, list: int list): int =
    let
        fun foldr' _ i [] = i
          | foldr' f i (x::xs) = f(x, (foldr' f i xs))
    in
        foldr' function initial list
    end
