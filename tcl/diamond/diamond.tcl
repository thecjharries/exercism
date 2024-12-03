proc diamond {letter} {
    set alphabet {A B C D E F G H I J K L M N O P Q R S T U V W X Y Z}
    set index [lsearch $alphabet $letter]
    if {$index < 0} {
        error "Invalid letter"
    }

    set height [expr {2 * $index + 1}]
    set result [lrepeat $height [lrepeat $height " "]]

    for {set i 0; set j $index} {$i <= $index} {incr i; incr j -1} {
        set letter [lindex $alphabet $i]
        lset result $i $j $letter
        lset result $i end-$j $letter
        lset result end-$i $j $letter
        lset result end-$i end-$j $letter
    }

    return [join [lmap row $result {join $row ""}] \n]
}
