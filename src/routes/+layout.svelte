<script lang="ts">
  import { useAuth } from '$lib/auth.svelte'
  import Toaster from '@/components/toaster.svelte'
  import '@/app.css'

  const auth = useAuth()

  let { children } = $props()

  $effect(() => {
    auth.checkConfigured()
  })
</script>

<Toaster />
{@render children()}
