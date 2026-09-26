<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { invoke } from '@tauri-apps/api/core'
  import Copy from 'phosphor-svelte/lib/CopyIcon'
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import GlobeSimple from 'phosphor-svelte/lib/GlobeSimpleIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import { usePasswords } from '$lib/password.svelte'
  import type { CardProps } from '@/components/types'
  import type { EntryPasswordProps } from '@/lib/types'

  const passwords = usePasswords()

  let { entry, onedit, ondelete }: CardProps<EntryPasswordProps> = $props()

  function getDomain(url: string): string {
    try {
      return new URL(url).hostname
    } catch {
      return url
    }
  }

  function getFavicon(url: string): string {
    try {
      const domain = new URL(url).hostname
      return `https://www.google.com/s2/favicons?domain=${domain}&sz=32`
    } catch {
      return ''
    }
  }

  async function copyPassword() {
    try {
      const decrypted = await passwords.decrypt(entry.id)
      await invoke('copy_to_clipboard', { text: decrypted })
      toast.success('Password copied to clipboard')
    } catch {
      toast.error('Failed to copy password')
    }
  }
</script>

<article
  class="flex items-center gap-x-4 rounded-card border-border-card bg-background-alt p-4 transition-colors hover:bg-muted"
>
  <div class="flex size-10 items-center justify-center rounded-lg bg-muted">
    {#if getFavicon(entry.url)}
      <img src={getFavicon(entry.url)} alt="{getDomain(entry.url)} favicon" class="size-6" />
    {:else}
      <GlobeSimple class="size-5 text-muted-foreground" />
    {/if}
  </div>

  <div class="min-w-0 flex-1">
    <div class="truncate text-sm font-medium text-foreground">
      {getDomain(entry.url)}
    </div>
    <div class="truncate text-xs text-foreground-alt">
      {entry.username}
    </div>
  </div>

  <div class="hidden items-center gap-1 text-muted-foreground sm:flex">
    <span class="text-xs tracking-widest">••••••••••</span>
  </div>

  <div class="flex items-center gap-x-1">
    <button
      title="Copy password"
      class="cursor-pointer rounded-lg p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
      onclick={copyPassword}
    >
      <Copy class="size-4" />
    </button>
    <button
      title="Edit"
      class="cursor-pointer rounded-lg p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
      onclick={() => onedit(entry)}
    >
      <PencilSimple class="size-4" />
    </button>
    <button
      title="Delete"
      class="cursor-pointer rounded-lg p-2 text-muted-foreground hover:bg-destructive/15 hover:text-destructive"
      onclick={() => ondelete(entry.id)}
    >
      <Trash class="size-4" />
    </button>
  </div>
</article>
