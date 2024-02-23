struct ListOps {
    static func append<T>(_ list1: [T], _ list2: [T]) -> [T] {
        return list1 + list2
    }

    static func concat<T>(_ args: [T]...) -> [T] {
        return args.reduce([], +)
    }

    static func filter<T>(_ list: [T], _ predicate: (T) -> Bool) -> [T] {
        return list.filter(predicate)
    }

    static func length<T>(_ list: [T]) -> Int {
        return list.count
    }

    static func map<T, U>(_ list: [T], _ transform: (T) -> U) -> [U] {
        return list.map(transform)
    }

    static func foldLeft<T, U>(_ list: [T], accumulated: U, combine: (U, T) -> U) -> U {
        return list.reduce(accumulated, combine)
    }

    static func foldRight<T, U>(_ list: [T], accumulated: U, combine: (T, U) -> U) -> U {
        return list.reversed().reduce(accumulated, combine)
    }

    static func reverse<T>(_ list: [T]) -> [T] {
        return list.reversed()
    }
}
