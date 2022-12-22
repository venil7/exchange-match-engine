import { pipe } from "fp-ts/lib/function";
import { chain, tryCatch } from "fp-ts/lib/TaskEither";
import { Action, ActionResult } from "../../domain/action";
import { fromJsError } from "../../domain/error";

const HEADERS: [string, string][] = [["content-type", "application/json"]];

const createFetch = (params: RequestInit = {}): Action<string, unknown> => {
  return (url: string) => {
    const result: ActionResult<unknown> = pipe(
      tryCatch(() => fetch(url, params), fromJsError),
      chain((resp) => tryCatch(() => resp.json(), fromJsError))
    );
    return result;
  };
};

export const fetchGet = createFetch({
  method: "GET",
  headers: HEADERS,
});

export const fetchPost = <T>(url: string, body: T) => {
  return pipe(
    url,
    createFetch({
      method: "POST",
      headers: HEADERS,
      body: JSON.stringify(body),
    })
  );
};
