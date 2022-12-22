<script lang="ts">
  import StoreData from "./components/StoreData.svelte";
  import Prices from "./components/Prices.svelte";
  import { prices } from "./stores/prices";
  import { orders } from "./stores/orders";
  import { Col, Row } from 'sveltestrap';
  import Orders from "./components/Orders.svelte";
</script>

<main>
  <Row>
    <Col>
      <StoreData store={prices} let:data={data}>
        <Prices data={data} />
      </StoreData>
    </Col>
    <Col>
      <StoreData store={orders} let:data={data}>
        <Orders data={data} />
      </StoreData>
    </Col>
  </Row>
    
</main>
