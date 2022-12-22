import { pipe } from "fp-ts/lib/function";
import { chain, of } from "fp-ts/lib/TaskEither";
import { map, right } from "fp-ts/lib/Either";
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
        chain((or) => of(update(map((ors) => [...ors, or]))))
      )();
    },
  };
};

export const orders = createOrders();
