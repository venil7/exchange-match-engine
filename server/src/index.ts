import bodyParser from "body-parser";
import express from "express";
import { Lazy, pipe } from "fp-ts/lib/function";
import * as TE from "fp-ts/lib/TaskEither";
import { fromJsError } from "./domain/error";
import { ApiEndpoint } from "./handler/api";
import { createGetEnv } from "./service/env";
import { createGetRedisApi } from "./service/redis";

// @ts-expect-error
global.server?.close();

const trySync = <A>(func: Lazy<A>) =>
  TE.tryCatch(async () => func(), fromJsError);

await pipe(
  TE.Do,
  TE.bind("env", () => createGetEnv("./conf.json")),
  TE.bind("redis", ({ env }) => createGetRedisApi(env)),
  TE.bind("app", (appCtx) =>
    TE.of(express().use(bodyParser.json()).use("/api", ApiEndpoint(appCtx)))
  ),
  TE.chain(({ app, env }) =>
    trySync(
      () =>
        //@ts-expect-error
        (global.server = app.listen(env.port, () =>
          console.log(`listening port ${env.port}, ${new Date()}`)
        ))
    )
  )
)();
