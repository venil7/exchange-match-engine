import { Either, tryCatch } from "fp-ts/lib/Either";
import { pipe } from "fp-ts/lib/function";
import { chain, fromEither, map, taskify, mapLeft } from "fp-ts/lib/TaskEither";
import fs from "fs";
import { EnvDecoder } from "../decoder/env";
import { ActionResult, fromDecoder } from "../domain/action";
import { Env } from "../domain/env";
import { AppError, fromJsError } from "../domain/error";

const parse = (s: string): Either<AppError, any> =>
  tryCatch<AppError, any>(() => JSON.parse(s), fromJsError);

export const createGetEnv = (path: string): ActionResult<Env> =>
  pipe(
    path,
    taskify(fs.readFile),
    mapLeft(fromJsError),
    map((b) => b.toString()),
    chain((s) => fromEither(parse(s))),
    chain(fromDecoder(EnvDecoder))
  );
