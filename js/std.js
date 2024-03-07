// id :: Va. a -> a
const id = (x) => x;

// add, sub, mul, div :: I32 -> I32 -> I32
const add = (x) => (y) => x + y;
const sub = (x) => (y) => x - y;
const mul = (x) => (y) => x * y;
const div = (x) => (y) => Math.floor(x / y);

// cmd :: Va. a -> Cmd a
const cmd = (x) => () => x;
// mapCmd :: Va, b. (a -> b) -> Cmd a -> Cmd b
const mapCmd = (f) => (c) => () => f(c());
// swapCmd :: Va, b. a -> Cmd b -> Cmd a
const swapCmd = (def) => (c) => () => {
  c();
  return def;
};
// thenCmd :: Va, b. Cmd a -> Cmd b -> Cmd b
const thenCmd = (ca) => (cb) => () => {
  ca();
  return cb();
};
// chainCmd :: Va, b. Cmd a -> (a -> Cmd b) -> Cmd b
const chainCmd = (ca) => (f) => f(ca());

// print :: Str -> Cmd Str
const print = (str) => () => {
  console.log(str);
  return str;
};

exports.id = id;
exports.add = add;
exports.sub = sub;
exports.mul = mul;
exports.div = div;
exports.cmd = cmd;
exports.mapCmd = mapCmd;
exports.swapCmd = swapCmd;
exports.thenCmd = thenCmd;
exports.chainCmd = chainCmd;
exports.print = print;
