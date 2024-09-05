using System.Text.RegularExpressions;

public static class PigLatin
{
    private static readonly Regex Vowel = new Regex(@"(?<start>^|\s+)(?<vowel>a|e|i|o|u|xr|yt)(?<end>\w+)", RegexOptions.Compiled);
    private static readonly Regex Consonant = new Regex(@"(?<start>^|\s+)(?<consonant>ch|qu|thr|th|rh|sch|yt|\wqu|\w)(?<end>\w+)", RegexOptions.Compiled);
    private const string VowelReplacement = "${start}${vowel}${end}ay";
    private const string ConsonantReplacement = "${start}${end}${consonant}ay";
    public static string Translate(string word) =>
        Vowel.IsMatch(word) ? Vowel.Replace(word, VowelReplacement) : Consonant.Replace(word, ConsonantReplacement);
}