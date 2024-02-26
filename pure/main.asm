main.main:
    NOP 0;
    PUSH_I32 42;
    RETURN;

main.p:
    NOP 0;
    PUSH_FN std.print;
    PUSH_I32 42;
    FEED 1;
    CALL;
    RETURN;

main.fst:
    NOP 2;
    PUSH_ARG 0;
    RETURN;

main.snd:
    NOP 2;
    PUSH_ARG 1;
    RETURN;
