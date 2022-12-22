<script lang="ts">
  import {
    Button,
    Card,
    CardBody, CardHeader,
    CardSubtitle, CardTitle
  } from 'sveltestrap';
  import type { AppError } from '../../domain/error';

  export let error: AppError;
</script>

<Card>
  <CardHeader>
    <CardTitle>{error.type} Error</CardTitle>
  </CardHeader>
  
  <CardBody>
    <CardSubtitle> {error.msg}</CardSubtitle>
  </CardBody>
</Card>