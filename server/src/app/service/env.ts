import { pipe } from "fp-ts/lib/function";
import { chain, fromEither, of } from "fp-ts/lib/TaskEither";
import { EnvDecoder } from "../../decoder/env";
import { ActionResult, fromDecoder } from "../../domain/action";
import { Env } from "../../domain/env";
import env from "../../../conf.json";

export const createGetEnv = (): ActionResult<Env> =>
  pipe(of(env), chain(fromDecoder(EnvDecoder)));
