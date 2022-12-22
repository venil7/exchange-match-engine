<script lang="ts">
  import { Card, CardBody, CardHeader, ListGroup, ListGroupItem } from 'sveltestrap';
  import type { OrderRequest } from "../../domain/order";
  
  export let data: OrderRequest[];
  // export let onAdd: (o:Order) => void;
  
</script>


<Card>
  <CardHeader>Orders</CardHeader>
  <CardBody>
    <!-- <OrderForm on:submit={onAdd} /> -->
    <ListGroup>
      {#if !data.length}no orders yet{/if}
      {#each data as order}
        <ListGroupItem>{order.direction}: {order.amount}@{order.price}</ListGroupItem>
      {/each}
    </ListGroup>
  </CardBody>
</Card>

