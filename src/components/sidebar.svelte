<script lang="ts">
  import { useAuth } from '$lib/auth.svelte'
  import Key from 'phosphor-svelte/lib/KeyIcon'
  import GearSix from 'phosphor-svelte/lib/GearSixIcon'
  import LockKey from 'phosphor-svelte/lib/LockKeyIcon'
  import SignOut from 'phosphor-svelte/lib/SignOutIcon'
  import { useNavigation } from '$lib/navigation.svelte'
  import type { SidebarProps } from '@/components/types'

  type NavPath = (typeof navItems)[number]['path']

  const auth = useAuth()
  const navigate = useNavigation()
  const settingsPath = '/settings'
  const navItems = [{ path: '/passwords', label: 'Passwords', icon: Key }] as const

  let { currentPath = '/' }: SidebarProps = $props()

  function handleNavigate(path: NavPath) {
    navigate.goto(`/(app)${path}`)
  }

  function handleSettingsNavigation() {
    navigate.goto(`/(app)${settingsPath}`)
  }

  function handleLock() {
    auth.lock()
  }
</script>

<aside class="flex h-full w-60 flex-col border-r border-dark-10">
  <header class="flex items-center gap-3 border-b border-dark-10 px-5 py-4">
    <div
      class="flex size-8 items-center justify-center rounded-lg bg-linear-to-br from-primary to-secondary text-white"
    >
      <LockKey class="size-4" />
    </div>
    <span class="text-lg font-bold text-white">Passnager</span>
  </header>

  <nav class="flex-1 flex flex-col gap-y-1 px-3 py-4">
    {#each navItems as item (item.path)}
      {@const Icon = item.icon}
      <button
        onclick={() => handleNavigate(item.path)}
        class={[
          'cursor-pointer flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors',
          { 'bg-primary/20 text-primary': currentPath === item.path },
          { 'text-foreground hover:bg-primary/20': currentPath !== item.path },
        ]}
      >
        <Icon class="size-5" />
        {item.label}
      </button>
    {/each}
  </nav>

  <footer class="flex flex-col gap-y-1 border-t border-dark-10 p-3">
    <button
      class={[
        'cursor-pointer flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors',
        { 'bg-primary/20 text-primary': currentPath === settingsPath },
        { 'text-foreground hover:bg-primary/20': currentPath !== settingsPath },
      ]}
      onclick={handleSettingsNavigation}
    >
      <GearSix class="size-5" />
      Settings
    </button>

    <button
      class="flex cursor-pointer w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-foreground transition-colors hover:bg-destructive/20 hover:text-destructive"
      onclick={handleLock}
    >
      <SignOut class="size-5" />
      Lock
    </button>
  </footer>
</aside>
