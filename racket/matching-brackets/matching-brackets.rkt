#lang racket

(provide balanced?)

(define (balanced? str)
    (define (is-balanced? lst openers)
        (if (empty? lst)
            (empty? openers)
            (let ([c (car lst)] [rest (cdr lst)])
                (match c
                    [#\( (is-balanced? rest (cons #\) openers))]
                    [#\{ (is-balanced? rest (cons #\} openers))]
                    [#\[ (is-balanced? rest (cons #\] openers))]
                    [( or #\) #\} #\]) (and (not (empty? openers))
                                            (equal? (car openers) c)
                                            (is-balanced? rest (cdr openers)))]
                    [_ (is-balanced? rest openers)]))))
    (is-balanced? (string->list str) '()))
