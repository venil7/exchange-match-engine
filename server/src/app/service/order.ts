import { pipe } from "fp-ts/lib/function";
import { chain } from "fp-ts/lib/TaskEither";
import { OrderRequestDecoder, OrderDecoder } from "../../decoder/order";
import { ActionResult, fromDecoder } from "../../domain/action";
import { Order, OrderRequest } from "../../domain/order";
import { createGetEnv } from "./env";
import { fetchGet, fetchPost } from "./fetch";

export const createGetOrder = (id: string): ActionResult<OrderRequest> =>
  pipe(
    createGetEnv(),
    chain(({ ticker, port }) =>
      fetchGet(`http://localhost:${port}/api/order/${ticker}/${id}`)
    ),
    chain(fromDecoder(OrderRequestDecoder))
  );

export const createPostOrder = (order: Order): ActionResult<OrderRequest> =>
  pipe(
    createGetEnv(),
    chain(({ ticker, port }) =>
      fetchPost(
        `http://localhost:${port}/api/order/${ticker}`,
        OrderDecoder.encode(order)
      )
    ),
    chain(fromDecoder(OrderRequestDecoder))
  );
