<?php

/*
 * By adding type hints and enabling strict type checking, code can become
 * easier to read, self-documenting and reduce the number of potential bugs.
 * By default, type declarations are non-strict, which means they will attempt
 * to change the original type to match the type specified by the
 * type-declaration.
 *
 * In other words, if you pass a string to a function requiring a float,
 * it will attempt to convert the string value to a float.
 *
 * To enable strict mode, a single declare directive must be placed at the top
 * of the file.
 * This means that the strictness of typing is configured on a per-file basis.
 * This directive not only affects the type declarations of parameters, but also
 * a function's return type.
 *
 * For more info review the Concept on strict type checking in the PHP track
 * <link>.
 *
 * To disable strict typing, comment out the directive below.
 */

declare(strict_types=1);

function meetup_day(int $year, int $month, string $which, string $weekday): DateTimeImmutable
{
    $teenth = false;
    if ($which === 'teenth') {
        $teenth = true;
        $which = 'first';
    }
    $month = DateTime::createFromFormat('!m', (string) $month);
    $monthWord = $month->format('F');
    $conversion = $which . ' ' . $weekday . ' of ' . $monthWord . ' ' . $year;
    $day = new DateTime($conversion);
    if ($teenth) {
        while ($day->format('d') < 13) {
            $day->modify('+1 week');
        }
    }
    return new DateTimeImmutable($day->format('Y-m-d'));
}
