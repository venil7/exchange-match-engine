<script lang="ts">
  import { LinkedChart } from "svelte-tiny-linked-charts";
  import { Card, CardBody, CardHeader } from "sveltestrap";

  import type { Price } from "../../domain/transaction";

  export let data: Price[];

  let labels: any;
  let values: number[];

  $: {
    labels = data.map((_, i) => i); //data.map(([label, _]) => label);
    values = data.map(([_, value]) => value);
  }
</script>

<Card>
  <CardHeader>Prices {data.length}</CardHeader>
  <CardBody>
    <LinkedChart
      {labels}
      {values}
      type="line"
      lineColor="#4355db"
      grow
      barMinWidth={0}
      width={500}
      height={200}
      fill="var(--text-color)"
    />
  </CardBody>
</Card>
