(module
    (import "console" "log" (func $log (param i32)))
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
        local.get $lhs
        local.get $rhs
        i32.add
    )
    (export "add" (func $add))
    (func $addAndLog (param $lhs i32) (param $rhs i32)
        local.get $lhs
        local.get $rhs
        (call $add (local.get $lhs) (local.get $rhs))
        (select)
        call $log
    )
    (export "addAndLog" (func $addAndLog))
)