namespace eval dnd {
    namespace export character ability modifier
    namespace ensemble create

    proc modifier {score} {
        expr {int(($score - 10) / 2)}
    }

    proc ability {} {
        expr {int(rand() * 16 + 3)}
    }

    proc character {} {
        set consitution [ability]
        return "charisma [ability] constitution $consitution dexterity [ability] intelligence [ability] strength [ability] wisdom [ability] hitpoints [expr {10 + [modifier $consitution]}]"
    }
}
