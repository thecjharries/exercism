maximum_value([], _, 0).
maximum_value(Items, Capacity, Value):-
    maximum_value(Items, Capacity, 0, Value).
maximum_value([], _, Value, Value):- Value > 0.
maximum_value([item(W,V)|Items], Capacity, Accumulator, Value):-
    W =< Capacity,
    NewAccumulator is Accumulator + V,
    NewCapacity is Capacity - W,
    maximum_value(Items, NewCapacity, NewAccumulator, Value).
maximum_value([_|Items], Capacity, Accumulator, Value):-
    maximum_value(Items, Capacity, Accumulator, Value).
