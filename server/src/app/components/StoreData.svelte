<script lang="ts">
  import { isRight } from "fp-ts/lib/Either";
  import type { Readable } from "svelte/store";
  import type { Result } from "../../domain/action";
  import Error from "./Error.svelte";
  
  export let store: Readable<Result<T>>;
  type T = $$Generic;
</script>

{#if isRight($store)}
  <slot data={$store.right} />
{:else}
  <Error error={$store.left}/>
{/if}