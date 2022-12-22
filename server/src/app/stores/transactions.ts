import { right } from "fp-ts/lib/Either";
import { readable } from "svelte/store";
import { Result } from "../../domain/action";
import { Tx } from "../../domain/transaction";
import { createGetTxs } from "../service/transactions";

export const transactions = readable<Result<Tx[]>>(right([]), (set) => {
  const getTxs = createGetTxs();

  (async () => {
    set(await getTxs());
  })();

  const interval = setInterval(async () => {
    set(await getTxs());
  }, 5000);

  return () => clearInterval(interval);
});
