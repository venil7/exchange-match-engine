import { isRight } from "fp-ts/lib/Either";
import { v4 as uuidv4 } from "uuid";
import { describe, expect, it as test } from "vitest";
import { OrderDecoder, OrderRequestDecoder } from "../decoder/order";
import { OrderDirection, OrderState } from "../domain/order";

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

const rawOrderRequest = {
  ...rawOrder,
  timestamp: new Date().toISOString(),
  id: uuidv4(),
  state: OrderState.Complete,
};

describe("order request", () => {
  test("decoder works", () => {
    let result = OrderRequestDecoder.decode(rawOrderRequest);
    expect(isRight(result)).toBe(true);
  });
});
