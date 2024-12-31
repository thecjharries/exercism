component {
	array function rows(letter) {
		var a = asc('A');
		var n = asc(letter) - a + 1;
		if (n < 1 || n > 26) return [];

		var diamond = [];
		var w = n * 2 - 1;
		for (var i = 0; i < n; i++) {
			var c = chr(a + i);
			var s = i == 0 ? c : c & ' '.repeat(i * 2 - 1) & c;
			diamond.append(s.cJustify(w));
		}
		return n == 1 ? diamond : diamond.merge(diamond.reverse().slice(2));
	}
}
