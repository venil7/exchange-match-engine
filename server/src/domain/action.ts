import { Decoder } from "io-ts";
import { AppError } from "./error";
import { pipe } from "fp-ts/lib/function";
import { Either, mapLeft } from "fp-ts/lib/Either";
import { fromValidationErrors } from "../domain/error";
import { ReaderTaskEither } from "fp-ts/lib/ReaderTaskEither";
import { TaskEither, fromEither } from "fp-ts/lib/TaskEither";

export type Result<A> = Either<AppError, A>;
export type ActionResult<A> = TaskEither<AppError, A>;
export type Action<Ctx, A> = ReaderTaskEither<Ctx, AppError, A>;

export const fromDecoder =
  <I, A>(decoder: Decoder<I, A>): ((i: I) => ActionResult<A>) =>
  (i: I) =>
    pipe(i, decoder.decode, mapLeft(fromValidationErrors), fromEither);
