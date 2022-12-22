<script lang="ts">
  import {
    Button,
    Card,
    CardBody,
    CardHeader,
    ListGroup,
    ListGroupItem,
  } from "sveltestrap";
  import type { Order, OrderRequest } from "../../domain/order";
  import { OrderDirection } from "../../domain/order";

  export let data: OrderRequest[];
  export let onAdd: (o: Order) => void;

  const randomOrder = (): Order => {
    return {
      amount: Math.round(Math.random() * 100),
      price: Math.round(Math.random() * 100),
      direction:
        Math.random() >= 0.5 ? OrderDirection.Sell : OrderDirection.Buy,
    };
  };
</script>

<Card>
  <CardHeader
    >Orders <Button on:click={() => onAdd(randomOrder())}>[+]</Button
    ></CardHeader
  >

  <CardBody>
    <ListGroup>
      {#if !data.length}no orders yet{/if}
      {#each data as order}
        <ListGroupItem
          >{order.direction}: {order.amount}@{order.price}</ListGroupItem
        >
      {/each}
    </ListGroup>
  </CardBody>
</Card>
