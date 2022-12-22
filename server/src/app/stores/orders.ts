import { append, takeRight } from "fp-ts/Array";
import { left, map, right } from "fp-ts/lib/Either";
import { pipe } from "fp-ts/lib/function";
import { match } from "fp-ts/lib/TaskEither";
import { writable } from "svelte/store";
import type { Result } from "../../domain/action";
import type { Order, OrderRequest } from "../../domain/order";
import { createPostOrder } from "../service/order";
const createOrders = () => {
  const { subscribe, set, update } = writable<Result<OrderRequest[]>>(
    right([])
  );
  return {
    subscribe,
    reset: () => set(right([])),
    submit: async (order: Order) => {
      await pipe(
        createPostOrder(order),
        match(
          (err) => set(left(err)),
          (ok) => update(map((ors) => pipe(ors, takeRight(10), append(ok))))
        )
      )();
    },
  };
};

export const orders = createOrders();
