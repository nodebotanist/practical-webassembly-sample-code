(module
    (func $calculate_stat_bonus (param $stat_modifier i32) (result i32)
        ;; create local to store result in
        (local $result i32)
        
        ;; load our stat modifier onto the stack
        local.get $stat_modifier
        ;; load zero onto the stack for comparison
        i32.const 0
        ;; check if the value is negative
        i32.lt_s ;; if negative, 1 on the stack, otherwise 0
        ;; use an if...then statement to branch on negative
        (if
            (then ;; value is negative
                ;; get the value on the stack
                local.get $stat_modifier
                ;; convert to f32
                f32.convert_i32_s
                ;; get absolute value
                f32.abs
                ;; truncate back to i32
                i32.trunc_f32_s
                ;; create a constant for the number of bits to shift
                ;; we always want to shift by one, so we use a constant
                i32.const 1
                ;; perform and store the bit shift:
                ;; pops the shift-by value off the stack
                ;; pops the stat_modifier off the stack
                ;; does the shift the pushes the result 
                ;; of the right shift onto the stack
                ;; which we then store in a local variable
                i32.shr_u
                local.set $result
                ;; push 0 onto the stack
                i32.const 0
                ;; push the result onto the stack
                local.get $result
                ;; subtract the absolute value of the modifier
                ;; from zero to make it negative and return
                i32.sub 
                local.set $result
            )
            (else ;; value is positive
                ;; all that needs to be done here is the bit shift
                ;; part
                local.get $stat_modifier
                i32.const 1
                i32.shr_u
                local.set $result
            )
        )
        local.get $result
        i32.const 5
        i32.sub
    )

    ;; export the calculate_stat_bonus function so it 
    ;; can be used by Javascript
    (export "calculate_stat_bonus" (func $calculate_stat_bonus))
)