Imports System

Public Class SpiralMatrix
    Public Shared Function GetMatrix(ByVal size As Integer) As Integer(,)
        Dim rowDirection As Integer() = {0, 1, 0, -1}
        Dim colDirection As Integer() = {1, 0, -1, 0}
        Dim directionIndex As Integer = 0
        Dim matrix As Integer(,) = New Integer(size - 1, size - 1) {}
        Dim row As Integer = 0
        Dim col As Integer = 0
        For number As Integer = 1 To size * size
            matrix(row, col) = number
            Dim nextRow As Integer = row + rowDirection(directionIndex)
            Dim nextCol As Integer = col + colDirection(directionIndex)
            If 0 <= nextRow AndAlso nextRow < size AndAlso 0 <= nextCol AndAlso nextCol < size AndAlso matrix(nextRow, nextCol) = 0 Then
                row = nextRow
                col = nextCol
            Else
                directionIndex = (directionIndex + 1) Mod 4
                row += rowDirection(directionIndex)
                col += colDirection(directionIndex)
            End If
        Next
        Return matrix
    End Function
End Class
