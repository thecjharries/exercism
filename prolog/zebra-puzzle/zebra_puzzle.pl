zebra_owner(Owner) :-
    houses(Hs),
    member(house(Owner,zebra,_,_,_), Hs).

water_drinker(Drinker) :-
    houses(Hs),
    member(house(Drinker,_,_,water,_), Hs).

houses(Hs) :-
    length(Hs, 5),
    member(house(englishman,_,_,_,red), Hs),
    member(house(spaniard,dog,_,_,_), Hs),
    member(house(_,_,_,coffee,green), Hs),
    member(house(ukrainian,_,_,tea,_), Hs),
    next_to(house(_,_,_,_,green), house(_,_,_,_,ivory), Hs),
    member(house(_,snails,old_gold,_,_), Hs),
    member(house(_,_,kool,_,yellow), Hs),
    Hs = [_,_,house(_,_,_,milk,_),_,_],
    Hs = [house(norwegian,_,_,_,_)|_],
    next_to(house(_,_,chesterfield,_,_), house(_,fox,_,_,_), Hs),
    next_to(house(_,_,kool,_,_), house(_,horse,_,_,_), Hs),
    member(house(_,_,lucky_strike,orange_juice,_), Hs),
    member(house(japanese,_,parliament,_,_), Hs),
    next_to(house(norwegian,_,_,_,_), house(_,_,_,_,blue), Hs),
    member(house(_,_,_,water,_), Hs),
    member(house(_,zebra,_,_,_), Hs).

next_to(A, B, L) :- append(_, [A,B|_], L).
next_to(A, B, L) :- append(_, [B,A|_], L).
