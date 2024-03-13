proc abbreviate {phrase} {
    set result ""
    set words [split $phrase " -_"]
    foreach word $words {
        set result "${result}[string toupper [string index $word 0]]"
    }
    return $result
}
