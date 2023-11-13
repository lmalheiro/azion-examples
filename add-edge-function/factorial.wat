(module
  ;; Recursive factorial function
  (func $factorial (param $n i32) (result i32)
    ;; Base case: if n is 0, return 1
    (if (result i32) (i32.eq (local.get $n) (i32.const 0))
      (then
        ;; Recursion base case
        (i32.const 1))
      (else
        ;; Recursive case: return n * factorial(n - 1)
        (i32.mul
          (local.get $n)
          (call $factorial
            (i32.sub (local.get $n) (i32.const 1))
          )
        )
      )
    )
  )
  ;; Export the factorial function with a named index
  (export "factorial" (func $factorial))
)
