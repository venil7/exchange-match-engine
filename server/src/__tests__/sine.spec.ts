import { reduce } from "fp-ts/lib/Array";
import { pipe } from "fp-ts/lib/function";
import { describe, expect, it as test } from "vitest";
import { sine } from "../app/service/util";

describe("sine generator", () => {
  test("total of 2pi period is zero", () => {
    let step = 0.001;
    let values = sine(step, 2 * (1 / step));
    let result = pipe(
      [...values],
      reduce(0, (acc, i) => acc + i)
    );
    expect(result).toBe(0);
  });
});
