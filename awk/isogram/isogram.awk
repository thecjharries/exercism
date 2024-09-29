BEGIN { PROCINFO["sorted_in"] = "@val_str_asc" }
{
    split($1, chars, "")
    word = ""
    for (i in chars) {
        char = chars[i]
        if (char ~ /[[:alpha:]]/) {
            char = tolower(char)
            if (char in seen) {
                print "false"
                next
            }
            seen[char] = 1
        }
        word = word char
    }
    print "true"
}
