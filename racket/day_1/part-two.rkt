#lang racket

(define test-data '("199" "200" "208" "210" "200" "207" "240" "269" "260" "263"))
(define test-values (map string->number test-data))

(define (windows acc lst n)
  (if (> n (length lst))
      (reverse acc)
      (windows (cons (take lst n) acc)
               (rest lst)
               n)))

(define (sum xs)
  (apply + xs))

(define (solve data)
  (let* ([ws (windows '() data 3)]
        [sums (map sum ws)]
        [sumwins (windows '() sums 2)])
    (count (lambda (p)
             (match-let ([(list a b) p])
               (> b a)))
           sumwins)))
    

(displayln (format "Test 5 = ~s" (solve test-values)))

(define live-data
  (call-with-input-file "input.txt"
    (lambda (in)
      (sequence->list (in-lines in)))))

(define live-values (map string->number live-data))

(displayln (format "PartTwo = ~s" (solve live-values)))