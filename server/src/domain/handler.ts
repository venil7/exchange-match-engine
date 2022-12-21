import {
  Handler,
  NextFunction,
  Request as ExpressRequest,
  Response as ExpressResponse,
} from "express";
import { isRight } from "fp-ts/lib/Either";
import { RedisApi } from "../service/redis";
import { Action } from "./action";
import { Env } from "./env";
import { ErrorType } from "./error";

export type AppContext = {
  redis: RedisApi;
  env: Env;
};

export type HandlerContext = {
  req: ExpressRequest;
  res: ExpressResponse;
  next?: NextFunction;
} & AppContext;

export type HandlerAction<T> = Action<HandlerContext, T>;

export const contextActionHandler =
  <T>(action: HandlerAction<T>) =>
  (ctx: AppContext): Handler => {
    return async (req, res, next) => {
      const task = action({ req, res, next, ...ctx });
      const result = await task();
      if (isRight(result)) return res.status(200).send(result.right);
      switch (result.left.type) {
        case ErrorType.NotFound:
          return res.status(404).send(result.left);
        case ErrorType.Validation:
          return res.status(406).send(result.left);
        default:
          return res.status(500).send(result.left);
      }
    };
  };
