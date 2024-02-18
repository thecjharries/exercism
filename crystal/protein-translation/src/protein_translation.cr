module ProteinTranslation
    def self.of_codon(codon : String) : String
        case codon
            when "AUG" then "Methionine"
            when "UUU", "UUC" then "Phenylalanine"
            when "UUA", "UUG" then "Leucine"
            when "UCU", "UCC", "UCA", "UCG" then "Serine"
            when "UAU", "UAC" then "Tyrosine"
            when "UGU", "UGC" then "Cysteine"
            when "UGG" then "Tryptophan"
            when "UAA", "UAG", "UGA" then "STOP"
            else raise ArgumentError.new("Invalid codon")
        end
    end

    def self.proteins(strand : String) : Array(String)
        strand
            .each_char
            .each_slice(3)
            .map { |codon| of_codon(codon.join) }
            .take_while { |protein| protein != "STOP" }
            .to_a
    end
end
