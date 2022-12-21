import { left } from "fp-ts/lib/Either";
import { Errors } from "io-ts";
import { PathReporter } from "io-ts/PathReporter";

export enum ErrorType {
  Generic,
  Validation,
  NotFound,
}
export type AppError = { type: ErrorType; msg?: string };

export const fromJsError = (e: unknown | Error): AppError => ({
  type: ErrorType.Generic,
  msg: (e as any).message ?? "unknown excpetion",
});

export const error = (type: ErrorType, msg = "error"): AppError => ({
  type,
  msg,
});

export const genericError = (msg: string) => error(ErrorType.Generic, msg);
export const notFound = (msg: string = "not found") =>
  error(ErrorType.NotFound, msg);

export const fromValidationErrors = (errors: Errors): AppError => ({
  type: ErrorType.Validation,
  msg: PathReporter.report(left(errors)).join(),
});
