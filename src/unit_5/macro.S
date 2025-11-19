.macro op_func label, a, b
    li a0, \a
    li a1, \b

    .if \label == 1
        call add_1
    .elseif \label == 2
        call add_2
    .else
        .error "Invalid label value"
    .endif
.endm