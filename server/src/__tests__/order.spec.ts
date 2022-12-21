import { isLeft, isRight } from "fp-ts/lib/Either";
import { assert, describe, expect, it as test } from "vitest";
import { OrderDecoder } from "../decoder/order";
import { Order, OrderDirection } from "../domain/order";

const rawOrder = {
  amount: 12,
  price: 10.0,
  direction: OrderDirection.Buy,
};

describe("order", () => {
  test("decoder works", () => {
    let result = OrderDecoder.decode(rawOrder);
    expect(isRight(result)).toBe(true);
  });
});
