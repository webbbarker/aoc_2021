#lang racket

(define (parse ls)
  (map (lambda (s)
         (let* ([parts (string-split s " ")]
                [action (first parts)]
                [amount (string->number (last parts))])
           (list action amount)))
       ls))

(define test-data '("forward 5" "down 5" "forward 8" "up 3" "down 8" "forward 2"))
(define test-values (parse test-data))

(define (traverse act pos)
  (let ([dist (car pos)]
        [depth (cadr pos)]
        [aim (caddr pos)])
    (match act
      [(list "forward" n) (list (+ dist n) (+ depth (* aim n)) aim)]
      [(list "up" n) (list dist depth (- aim n))]
      [(list "down" n) (list dist depth (+ aim n))])))

(define (solve vs)
  (apply * (take (foldl traverse '(0 0 0) vs) 2)))

(displayln (format "Test 900 = ~s" (solve test-values)))

(define live-data
  (call-with-input-file "racket/day_2/input.txt"
    (lambda (in)
      (sequence->list (in-lines in)))))

(define live-values (parse live-data))

(displayln (format "PartTwo = ~s" (solve live-values)))