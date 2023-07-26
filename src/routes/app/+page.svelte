<script lang="ts">
  import { user } from '$lib/stores/auth';
</script>

<div>Hello {$user?.name}</div>
