struct ListOps {
    static func append<T>(_ list1: [T], _ list2: [T]) -> [T] {
        return list1 + list2
    }

    static func concat<T>(_ args: [T]...) -> [T] {
        return args.reduce([], +)
    }
}
