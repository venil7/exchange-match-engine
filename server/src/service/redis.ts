import { pipe } from "fp-ts/lib/function";
import * as TE from "fp-ts/lib/TaskEither";
import * as t from "io-ts";
import { createClient } from "redis";
import { ActionResult, fromDecoder } from "../domain/action";
import { Env } from "../domain/env";
import { fromJsError, notFound } from "../domain/error";

type RedisClient = ReturnType<typeof createClient>;
export type RedisApi = {
  set: <T extends {}>(
    decoder: t.Type<T, string, string>
  ) => (key: string, obj: T) => ActionResult<T>;
  get: <T extends {}>(
    decoder: t.Type<T, string, string>
  ) => (key: string) => ActionResult<T>;
  enqueue: <T extends {}>(
    decoder: t.Type<T, string, string>
  ) => (key: string, obj: T) => ActionResult<T>;
  lrange: <T extends {}>(
    itemDecoder: t.Type<T, string>
  ) => (key: string, l?: number, r?: number) => ActionResult<T[]>;
};

const fromClient = (client: RedisClient): RedisApi => {
  return {
    set:
      <T extends {}>(decoder: t.Type<T, string, string>) =>
      (key: string, obj: T) => {
        return pipe(
          TE.of(decoder.encode(obj)),
          TE.chain((val) =>
            TE.tryCatch(() => client.set(key, val), fromJsError)
          ),
          TE.map(() => obj)
        );
      },

    get:
      <T extends {}>(decoder: t.Type<T, string, string>) =>
      (key: string) => {
        return pipe(
          TE.tryCatch(() => client.get(key), fromJsError),
          TE.chain(TE.fromNullable(notFound(`Record not found`))),
          TE.chain(fromDecoder(decoder))
        );
      },

    enqueue:
      <T extends {}>(decoder: t.Type<T, string, string>) =>
      (queueName: string, obj: T) => {
        return pipe(
          TE.of(decoder.encode(obj)),
          TE.chain((val) =>
            TE.tryCatch(() => client.lPush(queueName, val), fromJsError)
          ),
          TE.map(() => obj)
        );
      },

    lrange:
      <T extends {}>(itemDecoder: t.Type<T, string>) =>
      (key: string, l = 0, r = -1) => {
        return pipe(
          TE.tryCatch(() => client.lRange(key, l, r), fromJsError),
          TE.chain(fromDecoder(t.array(itemDecoder)))
        );
      },
  };
};

const connect = (client: RedisClient): ActionResult<RedisClient> => {
  return pipe(
    TE.tryCatch(() => client.connect(), fromJsError),
    TE.map(() => client)
  );
};

export const createGetRedisApi = (env: Env): ActionResult<RedisApi> => {
  const client = createClient({ url: env.redis });
  return pipe(TE.of(client), TE.chain(connect), TE.map(fromClient));
};
