main.start:
	NOP;
	PUSH_FN std.io.Print;
	PUSH_CMD main.zero;
	PUSH_CMD main.life;
	PUSH_BOOL 1;
	RETURN;

main.zero:
	NOP;
	PUSH_I32 0;
	RETURN;

/* this is just a comment */