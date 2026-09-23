<script lang="ts">
  import Toaster from '@/components/toaster.svelte'
  import '@/app.css'

  let { children } = $props()
</script>

<Toaster />
{@render children()}
