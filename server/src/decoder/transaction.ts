import * as t from "io-ts";
import { DateFromISOString, JsonFromString, UUID } from "io-ts-types";
import { OrderRequestDecoder } from "./order";

export const TxDecoder = t.type(
  {
    id: UUID,
    lhs: OrderRequestDecoder,
    rhs: OrderRequestDecoder,
    timestamp: DateFromISOString,
  },
  "tx"
);

export const TxFromStringDecoder = JsonFromString.pipe(TxDecoder);
