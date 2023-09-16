.number | "\(
    if 0 == . % 3
    then "Pling"
    else ""
    end
)\(
    if 0 == . % 5
    then "Plang"
    else ""
    end
)\(
    if 0 == . % 7
    then "Plong"
    else ""
    end
)\(
    if 0 != . % 3 and 0 != . % 5 and 0 != . % 7
    then "\(.)"
    else ""
    end
)"

