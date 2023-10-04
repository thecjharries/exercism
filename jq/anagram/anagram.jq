.subject as $subject |
  ($subject | ascii_downcase | split("") | sort | join("")) as $sortedSubject |
  reduce .candidates[] as $candidate (
    [];
    if ($candidate | ascii_downcase | split("") | sort | join("")) == $sortedSubject and ($subject | ascii_downcase) != ($candidate | ascii_downcase) then
      . + [$candidate]
    else
      .
    end
  )
