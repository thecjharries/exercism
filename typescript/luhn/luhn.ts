export function valid(digitString: string): unknown {
    if (
        !/^[0-9 ]+$/.test(digitString) ||
        digitString.replace(/\s/g, "").length < 2
    ) {
        return false;
    }
    const sum = [...digitString]
        .filter((char) => /\d/.test(char))
        .reverse()
        .map(Number)
        .reduce((acc, digit, index) => {
            if (index % 2 === 1) {
                if (digit * 2 > 9) {
                    return acc + digit * 2 - 9;
                }
                return acc + digit * 2;
            }
            return acc + digit;
        }, 0);
    return sum % 10 === 0;
}
