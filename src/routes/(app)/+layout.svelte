<script lang="ts">
  import { page } from '$app/state'
  import type { RouteId } from '$app/types'
  import { useAuth } from '$lib/auth.svelte'
  import Sidebar from '@/components/sidebar.svelte'
  import { useNavigation } from '$lib/navigation.svelte'
  import '@/app.css'

  const auth = useAuth()
  const navigate = useNavigation()

  let { children } = $props()

  let currentPath = $derived(page.url.pathname)

  function handleLock() {
    void auth.lock()
  }

  function handleNavigate(path: string) {
    navigate.goto(`/(app)${path}` as RouteId)
  }

  $effect(() => {
    if (!auth.isConfigured) navigate.goto('/setup')
    else if (!auth.isUnlocked) navigate.goto('/unlock')
  })
</script>

{#if auth.isConfigured && auth.isUnlocked}
  <div class="flex h-screen">
    <Sidebar {currentPath} onlock={handleLock} onnavigate={handleNavigate} />
    <main class="flex-1 overflow-auto p-6">
      {@render children()}
    </main>
  </div>
{/if}
