<script lang="ts">
  import { Toaster } from 'svelte-sonner'
  import { useTheme } from '$lib/theme.svelte'

  const theme = useTheme()
</script>

<Toaster theme={theme.value} position="bottom-right" richColors />
