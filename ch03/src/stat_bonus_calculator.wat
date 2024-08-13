(module
    (func $calculate_stat_bonus (param $stat_modifier i32) (result i32)
        ;; load our stat modifier onto the stack
        local.get $stat_modifier
        ;; we always want to shift by one, so we use a constant
        i32.const 1
        ;; perform and return the bit shift:
        ;; pops the shift-by value off the stack
        ;; pops the stat_modifier off the stack
        ;; does the shift the pushes the result 
        ;; of the right shift onto the stack and returns
        i32.shr_u
    )
    ;; export the calculate_stat_bonus function so it 
    ;; can be used by Javascript
    (export "calculate_stat_bonus" (func $calculate_stat_bonus))
)