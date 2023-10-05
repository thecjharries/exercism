.phrase | ascii_downcase | gsub("[^a-z]"; "") | split("") | (sort == unique)
