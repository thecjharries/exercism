# These variables are initialized on the command line (using '-v'):
# - name

BEGIN {
    FS = ""
    n = split("Alice Bob Charlie David Eve Fred Ginny Harriet Ileana Joseph Kincaid Larry", names, " ")
    for (i = 1; i <= n; i++) {
        if (toupper(names[i]) == toupper(name)) {
            offset = i
        }
    }
    n = split("violets radishes clover grass", plants, " ")
    for (i = 1; i <= n; i++) {
        plant[toupper(substr(plants[i], 1, 1))] = plants[i]
    }
}
{
    out = out " " plant[$(2 * offset - 1)] " " plant[$(2 * offset)];
}
END {
    print substr(out, 2)
}
