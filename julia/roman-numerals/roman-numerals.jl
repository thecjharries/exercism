function to_roman(number)
    if number <= 0 || number >= 4000
        throw(ErrorException("Number must be between 1 and 3999"))
    end
    roman = ""
    for (arabic, numeral) in [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
                              (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
                              (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")]
        while number >= arabic
            roman *= numeral
            number -= arabic
        end
    end
    return roman
end
