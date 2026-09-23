<script lang="ts">
  import { useAuth } from '$lib/auth.svelte'
  import { useNavigation } from '$lib/navigation.svelte'
  import Loading from '@/components/loading.svelte'

  const auth = useAuth()
  const navigate = useNavigation()

  $effect(() => {
    if (auth.isConfigured === null) return
    if (!auth.isConfigured) {
      navigate.goto('/setup')
    } else if (!auth.isUnlocked) {
      navigate.goto('/unlock')
    } else {
      navigate.goto('/(app)/passwords')
    }
  })
</script>

<main class="w-full h-screen grid place-items-center">
  <Loading />
</main>
