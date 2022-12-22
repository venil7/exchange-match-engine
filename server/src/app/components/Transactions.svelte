<script lang="ts">
  import { LinkedChart, LinkedLabel, LinkedValue } from "svelte-tiny-linked-charts";


  export let labels: Date[];
  export let values: number[];
</script>

<LinkedChart
  {labels} {values}
  lineColor="#4355db"
  type="line"
  fill="var(--text-color)" 
/>