import express from "express";
import { pipe } from "fp-ts/lib/function";
import TE from "fp-ts/lib/TaskEither";
import { Type } from "io-ts";
import {
  decodeOrder,
  decodeOrderId,
  OrderRequestFromStringDecoder,
} from "../decoder/order";
import { TxFromStringDecoder } from "../decoder/transaction";
import { Action } from "../domain/action";
import {
  AppContext,
  contextActionHandler,
  HandlerContext,
} from "../domain/handler";
import { Order, enrichPendingOrderRequest } from "../domain/order";
import { Tx } from "../domain/transaction";

const orderGetAction: Action<HandlerContext, Order> = ({
  redis,
  req,
  env,
}: HandlerContext) =>
  pipe(
    req.params["id"],
    decodeOrderId,
    TE.chain((id) =>
      redis.get(OrderRequestFromStringDecoder)(`${env.ticker}/order/${id}`)
    )
  );

const txsGetAction: Action<HandlerContext, Tx[]> = ({
  redis,
  env,
}: HandlerContext) => {
  return pipe(
    // fetch last 100 items of the list
    redis.lrange(TxFromStringDecoder as Type<Tx, string>)(
      `${env.ticker}/txs`,
      -100,
      -1
    )
  );
};

const orderPushAction: Action<HandlerContext, Order> = ({
  redis,
  req,
  env,
}: HandlerContext) =>
  pipe(
    req.body,
    decodeOrder,
    TE.map(enrichPendingOrderRequest),
    TE.chain((order) =>
      redis.set(OrderRequestFromStringDecoder)(
        `${env.ticker}/order/${order.id}`,
        order
      )
    ),
    TE.chain((order) =>
      redis.enqueue(OrderRequestFromStringDecoder)(
        `${env.ticker}/orders`,
        order
      )
    )
  );

const orderPostAction: Action<HandlerContext, Order> = ({
  redis,
  req,
  env,
}: HandlerContext) =>
  pipe(
    req.body,
    decodeOrder,
    TE.map(enrichPendingOrderRequest),
    TE.chain((order) =>
      redis.set(OrderRequestFromStringDecoder)(
        `${env.ticker}/order/${order.id}`,
        order
      )
    )
  );

const orderGetHandler = contextActionHandler(orderGetAction);
const txsGetHandler = contextActionHandler(txsGetAction);
const orderPostHandler = contextActionHandler(orderPostAction);
const orderPushHandler = contextActionHandler(orderPushAction);

export const ApiEndpoint = (appCtx: AppContext) => {
  const {
    env: { ticker },
  } = appCtx;
  console.info(`The following endpoints available:
    POST /order/${ticker} 
    GET  /order/${ticker}/:id 
    GET  /tx/${ticker}
  `);
  return express()
    .post(`/order/${ticker}`, orderPushHandler(appCtx))
    .get(`/order/${ticker}/:id`, orderGetHandler(appCtx))
    .get(`/tx/${ticker}`, txsGetHandler(appCtx));
  // .post(`/order/${ticker}`, orderPostHandler(appCtx))
};
