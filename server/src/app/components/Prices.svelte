<script lang="ts">
  import { LinkedChart } from "svelte-tiny-linked-charts";
  import { Card, CardBody, CardHeader } from 'sveltestrap';

  import type { Price } from "../stores/prices";
  
  export let data: Price[];
  
  let labels: Date[];
  let values: number[];

  $: {
    labels = data.map(([label, _]) => label)
    values = data.map(([_, value]) => value)
  }
</script>


<Card>
  <CardHeader>Prices</CardHeader>
  <CardBody>
    <LinkedChart
      {labels} {values}
      lineColor="#4355db"
      type="line"
      fill="var(--text-color)" 
    />
  </CardBody>
</Card>

