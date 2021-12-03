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
  (let ([dist (first pos)]
        [depth (last pos)])
    (match act
      [(list "forward" n) (list (+ dist n) depth)]
      [(list "up" n) (list dist (- depth n))]
      [(list "down" n) (list dist (+ depth n))])))

(define (solve vs)
  (apply * (foldl traverse '(0 0) vs)))

(displayln (format "Test 150 = ~s" (solve test-values)))

(define live-data
  (call-with-input-file "racket/day_2/input.txt"
    (lambda (in)
      (sequence->list (in-lines in)))))

(define live-values (parse live-data))

(displayln (format "PartOne = ~s" (solve live-values)))