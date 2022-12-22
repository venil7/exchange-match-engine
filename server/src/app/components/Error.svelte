<script lang="ts">
  import {
    Card,
    CardBody, CardHeader,
    CardSubtitle, CardTitle
  } from 'sveltestrap';
  import type { AppError } from '../../domain/error';

  export let error: AppError;
</script>

<Card class="mb-3">
  <CardHeader>
    <CardTitle>{error.type} Error</CardTitle>
  </CardHeader>
  <CardBody>
    <CardSubtitle> {error.msg}</CardSubtitle>
  </CardBody>
</Card>