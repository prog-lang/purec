const std = require("./std");

function main() {
  return std.prints(magic(40));
}

function magic(x) {
  return std.mul(x)(42);
}
