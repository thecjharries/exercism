using System;

public static class ProteinTranslation
{
    public static string FromCodon(string codon)
    {
        switch (codon)
        {
            case "AUG":
                return "Methionine";
            case "UUU":
            case "UUC":
                return "Phenylalanine";
            case "UUA":
            case "UUG":
                return "Leucine";
            case "UCU":
            case "UCC":
            case "UCA":
            case "UCG":
                return "Serine";
            case "UAU":
            case "UAC":
                return "Tyrosine";
            case "UGU":
            case "UGC":
                return "Cysteine";
            case "UGG":
                return "Tryptophan";
            case "UAA":
            case "UAG":
            case "UGA":
                return "STOP";
            default:
                throw new Exception("Invalid codon");
        }
    }

    public static string[] Proteins(string strand)
    {
        string[] proteins = new string[strand.Length / 3];
        for (int i = 0; i < strand.Length; i += 3)
        {
            string codon = strand.Substring(i, 3);
            string protein = FromCodon(codon);
            if (protein == "STOP")
            {
                Array.Resize(ref proteins, i / 3);
                break;
            }
            proteins[i / 3] = protein;
        }
        return proteins;
    }
}