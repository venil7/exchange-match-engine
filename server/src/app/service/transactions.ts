import { pipe } from "fp-ts/lib/function";
import { chain } from "fp-ts/lib/TaskEither";
import { TxsDecoder } from "../../decoder/transaction";
import { ActionResult, fromDecoder } from "../../domain/action";
import { Tx } from "../../domain/transaction";
import { createGetEnv } from "./env";
import { fetchGet } from "./fetch";

export const createGetTxs = (): ActionResult<Tx[]> => {
  return pipe(
    createGetEnv(),
    chain(({ ticker, port }) =>
      fetchGet(`http://localhost:${port}/api/tx/${ticker}`)
    ),
    chain(fromDecoder(TxsDecoder))
  );
};
