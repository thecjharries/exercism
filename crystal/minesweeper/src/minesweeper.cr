class Minesweeper
  def initialize(@board : Array(String))
  end

  def annotate
    @board.map_with_index do |row, y|
      row.chars.map_with_index do |cell, x|
        cell.to_s.tr(" ", count_mines(x, y))
      end.join
    end
  end

  private def count_mines(x, y)
    ([y - 1, 0].max..[y + 1, @board.size - 1].min).map do |i|
      ([x - 1, 0].max..[x + 1, @board[0].size - 1].min).count do |j|
        @board[i][j] == '*'
      end
    end.sum.to_s.tr("0", " ")
  end
end
