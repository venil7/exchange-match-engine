import { append, takeRight } from "fp-ts/Array";
import { left, map, right } from "fp-ts/lib/Either";
import { pipe } from "fp-ts/lib/function";
import { match } from "fp-ts/lib/TaskEither";
import { writable } from "svelte/store";
import type { Result } from "../../domain/action";
import { Order, OrderDirection, OrderRequest } from "../../domain/order";
import { createPostOrder } from "../service/order";
import { randomInt, sine } from "../service/util";
const createOrders = () => {
  const { subscribe, set, update } = writable<Result<OrderRequest[]>>(
    right([])
  );

  const reset = () => set(right([]));
  const submit = async (order: Order) => {
    return pipe(
      createPostOrder(order),
      match(
        (err) => set(left(err)),
        (ok) => update(map((ors) => pipe(ors, takeRight(9), append(ok))))
      )
    )();
  };

  const generate = async () => {
    let step = 0.01;
    let values = sine(step, 6 * (1 / step));
    for (let v of values) {
      const positive = v ** 2 + 1 + randomInt(-1, +1);
      const buyOrder: Order = {
        price: positive,
        amount: 1,
        direction: OrderDirection.Buy,
      } as const;
      await submit(buyOrder);
      const sellOrder: Order = {
        price: positive,
        amount: randomInt(1, 10),
        direction: OrderDirection.Sell,
      } as const;
      await submit(sellOrder);
    }
  };

  return {
    subscribe,
    generate,
    reset,
    submit,
  };
};

export const orders = createOrders();
