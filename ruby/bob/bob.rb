class Bob
    def self.hey(s)
        @s = s.strip
        if (@s == @s.upcase) && @s[-1] == '?' && s.count("a-zA-Z") > 0
            return "Calm down, I know what I'm doing!"
        elsif @s == @s.upcase && s.count("a-zA-Z") > 0
            return "Whoa, chill out!"
        elsif @s[-1] == '?'
            return "Sure."
        elsif @s =~ /\A\s*\Z/
            return "Fine. Be that way!"
        else
            return "Whatever."
        end
    end
end
