// user input is an Option<Pattern>
// need a way to expand ranges into a Paren
// each non-Composite pattern has a size "{n}" (or implicitly 1)
// this is just the number of sampling iterations
// we don't need for Composite patterns to have a size other than 1
// (the size should "distribute")
enum Pattern {
	Composite, // e.g., "(a|b)c", "abc*" ("*" is for special characters)
	Literal, // e.g., "abc"
	Paren, // e.g., "(a|b|c)", "(aaa|bbb)"
	Bracket, // e.g., "[abc]"
};
