<script lang="ts">
  import { prices } from "./stores/prices";
  import Transactions from "./components/Transactions.svelte"
  import { isRight } from "fp-ts/lib/Either";

  let labels: Date[] = [];
  let values: number[] = [];

  $: {
    prices.subscribe(res => {
      if (isRight(res)) {
        labels = res.right.map(([label, _]) => label)
        values = res.right.map(([_, value]) => value)
      }
    })
  }
</script>

<main>
 <Transactions {labels} {values} />
</main>
