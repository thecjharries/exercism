return function(phrase)
    if string.find(phrase, "^[aeiou]") or string.find(phrase, "^xr") or string.find(phrase, "^yt") then
        return phrase .. "ay"
    else
        return string.gsub(phrase, "([^aeiou%s]+u?)([aeiouy]%a*)", "%2%1ay")
    end
end
