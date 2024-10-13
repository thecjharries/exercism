-module(meetup).

-export([meetup/4]).

-type year() :: integer().
-type month() :: 1..12.
-type day() :: 1..31.
-type date() :: {year(), month(), day()}.
-type day_of_week() :: monday | tuesday | wednesday | thursday | friday | saturday | sunday.
-type week_of_month() :: first | second | third | fourth | teenth | last.

-define (DAY_OF_WEEK_MAP, #{
    monday => 1,
    tuesday => 2,
    wednesday => 3,
    thursday => 4,
    friday => 5,
    saturday => 6,
    sunday => 7}).

-spec meetup(year(), month(), day_of_week(), week_of_month()) -> date().
meetup(Year, Month, DayOfWeek, WeekOfMonth) ->
    DofW = maps:get(DayOfWeek, ?DAY_OF_WEEK_MAP),
    [Day] = [Day || Day <- day_in_week_of_month(WeekOfMonth, Year, Month),
                    calendar:day_of_the_week({Year, Month, Day}) =:= DofW],
    {Year, Month, Day}.

day_in_week_of_month(first, _, _) -> lists:seq(1, 7);
day_in_week_of_month(second, _, _) -> lists:seq(8, 14);
day_in_week_of_month(third, _, _) -> lists:seq(15, 21);
day_in_week_of_month(fourth, _, _) -> lists:seq(22, 28);
day_in_week_of_month(teenth, _, _) -> lists:seq(13, 19);
day_in_week_of_month(last, Year, Month) ->
    LastDay = calendar:last_day_of_the_month(Year, Month),
    lists:seq(LastDay - 6, LastDay).
