<script lang="ts">
  import StoreData from "./components/StoreData.svelte";
  import Prices from "./components/Prices.svelte";
  import { prices } from "./stores/prices";
  import { orders } from "./stores/orders";
  import { Col, Row } from "sveltestrap";
  import Orders from "./components/Orders.svelte";
</script>

<main>
  <Row class="mt-3">
    <Col>
      <StoreData store={prices} let:data>
        <Prices {data} />
      </StoreData>
    </Col>
    <Col>
      <StoreData store={orders} let:data>
        <Orders {data} onAdd={orders.submit} onGenerate={orders.generate} />
      </StoreData>
    </Col>
  </Row>
</main>
