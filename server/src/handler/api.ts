import express from "express";
import { pipe } from "fp-ts/lib/function";
import TE from "fp-ts/lib/TaskEither";
import {
  decodeOrder,
  decodeOrderId,
  OrderRequestFromStringEncoder,
} from "../decoder/order";
import { Action } from "../domain/action";
import {
  AppContext,
  contextActionHandler,
  HandlerContext,
} from "../domain/handler";
import { Order, enrichOrderRequest } from "../domain/order";

const orderGetAction: Action<HandlerContext, Order> = ({
  redis,
  req,
  env,
}: HandlerContext) =>
  pipe(
    req.params["id"],
    decodeOrderId,
    TE.chain((id) =>
      redis.get(OrderRequestFromStringEncoder)(`${env.ticker}/order/${id}`)
    )
  );

const orderPushAction: Action<HandlerContext, Order> = ({
  redis,
  req,
  env,
}: HandlerContext) =>
  pipe(
    req.body,
    decodeOrder,
    TE.map(enrichOrderRequest),
    TE.chain((order) =>
      redis.set(OrderRequestFromStringEncoder)(
        `${env.ticker}/order/${order.id}`,
        order
      )
    ),
    TE.chain((order) =>
      redis.enqueue(OrderRequestFromStringEncoder)(
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
    TE.map(enrichOrderRequest),
    TE.chain((order) =>
      redis.set(OrderRequestFromStringEncoder)(
        `${env.ticker}/order/${order.id}`,
        order
      )
    )
  );

const orderGetHandler = contextActionHandler(orderGetAction);
const orderPostHandler = contextActionHandler(orderPostAction);
const orderPushHandler = contextActionHandler(orderPushAction);

export const ApiEndpoint = (appCtx: AppContext) =>
  express()
    .get(`/order/${appCtx.env.ticker}/:id`, orderGetHandler(appCtx))
    // .post(`/order/${appCtx.env.ticker}`, orderPostHandler(appCtx))
    .post(`/order/${appCtx.env.ticker}`, orderPushHandler(appCtx));
