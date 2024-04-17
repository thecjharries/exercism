BEGIN { FPAT = "[0-9]" }
{
    sum = 0;
    for (i = 1; i <= NF; i++) {
        if ((NF - i) % 2 == 0) {
            sum += $i;
        } else {
            sum += $i * 2 > 9 ? $i * 2 - 9 : $i * 2;
        }
    }
    print (!/[^0-9 ]/ && sum % 10 == 0 && NF > 1) ? "true" : "false";
}
