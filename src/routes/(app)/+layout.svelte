<script lang="ts">
  import { page } from '$app/state'
  import type { RouteId } from '$app/types'
  import { useAuth } from '$lib/auth.svelte'
  import Sidebar from '@/components/sidebar.svelte'
  import { useNavigation } from '$lib/navigation.svelte'
  import CheckError from '@/components/check-error.svelte'


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

  function handleRetry() {
    void auth.retryCheck()
  }

  $effect(() => {
    // `null` is "the check has not answered yet", and it is not the same as
    // `false`: `!null === true`, so reading it as unconfigured redirected to
    // `/setup` while the query was still in flight — and `/setup` has no way back
    // to `/unlock`, so a slow answer was enough to strand the user there until
    // they restarted the app. Waiting is the whole fix.
    if (auth.isConfigured === null) return

    if (!auth.isConfigured) navigate.goto('/setup')
    else if (!auth.isUnlocked) navigate.goto('/unlock')
  })
</script>

{#if auth.configuredError}
  <CheckError message={auth.configuredError} onretry={handleRetry} />
{:else if auth.isConfigured && auth.isUnlocked}
  <div class="flex h-screen">
    <Sidebar {currentPath} onlock={handleLock} onnavigate={handleNavigate} />
    <main class="flex-1 overflow-auto p-6">
      {@render children()}
    </main>
  </div>
{/if}
