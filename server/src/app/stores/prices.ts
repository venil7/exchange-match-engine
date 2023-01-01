import { map } from "fp-ts/lib/Either";
import { map as amap } from "fp-ts/lib/Array";
import { pipe } from "fp-ts/lib/function";
import { derived } from "svelte/store";
import { transactions } from "./transactions";
import { Result } from "../../domain/action";
import { Price, toPrice } from "../../domain/transaction";

export const prices = derived<typeof transactions, Result<Price[]>>(
  transactions,
  ($txs) => pipe($txs, map(amap(toPrice)))
);
