// id :: Va. a -> a
id = (x) => x;
// iff :: Va. Bool -> a -> a -> a
iff = (cond) => (x) => (y) => cond ? x : y;

// add, sub, mul, div :: I32 -> I32 -> I32
add = (x) => (y) => x + y;
sub = (x) => (y) => x - y;
mul = (x) => (y) => x * y;
div = (x) => (y) => Math.floor(x / y);

// cmd :: Va. a -> Cmd a
cmd = (x) => () => x;
// mapCmd :: Va, b. (a -> b) -> Cmd a -> Cmd b
mapCmd = (f) => (c) => () => f(c());
// swapCmd :: Va, b. a -> Cmd b -> Cmd a
swapCmd = (def) => (c) => () => {
  c();
  return def;
};
// thenCmd :: Va, b. Cmd a -> Cmd b -> Cmd b
thenCmd = (ca) => (cb) => () => {
  ca();
  return cb();
};
// chainCmd :: Va, b. Cmd a -> (a -> Cmd b) -> Cmd b
chainCmd = (ca) => (f) => () => f(ca())();

// prints :: Str -> Cmd Str
prints = (str) => () => {
  console.log(str);
  return str;
};

module.exports = {
  id,
  iff,
  add,
  sub,
  mul,
  div,
  cmd,
  mapCmd,
  swapCmd,
  thenCmd,
  chainCmd,
  prints,
};
