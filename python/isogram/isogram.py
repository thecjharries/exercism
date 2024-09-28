def is_isogram(string):
    input = "".join(filter(str.isalpha, string)).lower()
    return len(input) == len(set(input))
