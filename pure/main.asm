main.main:
    PUSH_I32 42;
    RETURN;

main.p:
    PUSH_FN std.print;
    PUSH_I32 42;
    FEED 1;
    CALL;
    RETURN;

main.fst:
    PUSH_ARG 0;
    RETURN;

main.snd:
    PUSH_ARG 1;
    RETURN;
