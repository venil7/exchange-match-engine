import * as t from "io-ts";
import { DateFromISOString, JsonFromString, UUID } from "io-ts-types";
import { fromDecoder } from "../domain/action";
import { OrderDirection } from "../domain/order";

export const OrderDirectionDecoder = t.union(
  [t.literal(OrderDirection.Buy), t.literal(OrderDirection.Sell)],
  "orderType"
);

// used by REST endpoint to accept new orders
export const OrderDecoder = t.type(
  {
    price: t.number,
    amount: t.number,
    direction: OrderDirectionDecoder,
  },
  "order"
);

// used by REDIS to queue new orders into the engine,
// is an extension of OrderDecoder
export const OrderRequestDecoder = t.type(
  {
    id: UUID,
    ...OrderDecoder.props,
    timestamp: DateFromISOString,
  },
  "orderRequest"
);

// export const OrderFromStringDecoder = JsonFromString.pipe(OrderDecoder);
export const OrderRequestFromStringEncoder =
  JsonFromString.pipe(OrderRequestDecoder);
export const decodeOrderId = fromDecoder(t.string);
export const decodeOrder = fromDecoder(OrderDecoder);
// export const decodeOrderFromString = fromDecoder(OrderFromStringDecoder);
export const decodeOrderRequestFromString = fromDecoder(
  OrderRequestFromStringEncoder
);
