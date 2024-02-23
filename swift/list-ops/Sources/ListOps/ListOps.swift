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

    static func foldLeft<T>(_ list: [T], accumulated: T, combine: (T, T) -> T) -> T {
        return list.reduce(accumulated, combine)
    }

    static func foldRight<T>(_ list: [T], accumulated: T, combine: (T, T) -> T) -> T {
        var result = accumulated
        var list = list
        for _ in list {
            result = combine(list.last!, result)
            list = list.dropLast()
        }
        return result
    }

    static func reverse<T>(_ list: [T]) -> [T] {
        return list.reversed()
    }
}
