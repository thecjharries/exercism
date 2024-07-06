Imports System.Collections.Generic

Public Module MatchingBrackets
    Private ReadOnly bracketPairs As New Dictionary(Of Char, Char) From {
        {"[", "]"},
        {"{", "}"},
        {"(", ")"}
    }

    Public Function IsPaired(ByVal input As String) As Boolean
        Dim stack As New Stack(Of Char)

        For Each c As Char In input
            If bracketPairs.ContainsKey(c) Then
                stack.Push(c)
            ElseIf bracketPairs.ContainsValue(c) Then
                If stack.Count = 0 OrElse bracketPairs(stack.Pop()) <> c Then
                    Return False
                End If
            End If
        Next

        Return stack.Count = 0
    End Function
End Module
