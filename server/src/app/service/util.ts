import { pipe } from "fp-ts/lib/function";

export function* sine(step = 0.01, take = Infinity) {
  let phi = 0,
    c = 0;
  while (c <= take) {
    yield pipe(phi * Math.PI, Math.sin, (n) => n * 100, Math.round);
    phi += step;
    c += 1;
  }
}

export const randomInt = (min = 0, max = 100) => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
};
