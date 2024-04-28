"""
    ispangram(input)

Return `true` if `input` contains every alphabetic character (case insensitive).

"""
function ispangram(input)
    alphabet = Set('a':'z')
    return alphabet ⊆ Set(lowercase(input))
end

