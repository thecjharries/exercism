object Series {
    def largestProduct(len: Int, digits: String): Option[Int] =
        if (0 > len || len > digits.length || digits.exists(!_.isDigit)) None
        else if (0 == len) Some(1)
        else
            Some((0 to digits.length - len).map(x => digits.substring(x, x + len).map(_.toString.toInt).reduce(_ * _)).max)
}
