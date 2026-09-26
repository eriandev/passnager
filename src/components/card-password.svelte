<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { invoke } from '@tauri-apps/api/core'
  import Copy from 'phosphor-svelte/lib/CopyIcon'
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import GlobeSimple from 'phosphor-svelte/lib/GlobeSimpleIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import CardAction from '@/components/card-action.svelte'
  import { usePasswords } from '$lib/password.svelte'
  import type { CardProps } from '@/components/types'
  import type { EntryPasswordProps } from '@/lib/types'

  const actions = [
    {
      icon: Copy,
      title: 'Copy password',
      action: copyPassword,
    },
    {
      title: 'Edit',
      icon: PencilSimple,
      action: () => onedit(entry),
    },
    {
      icon: Trash,
      title: 'Delete',
      action: () => ondelete(entry.id),
    },
  ] as const
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
  <section class="flex size-10 items-center justify-center rounded-lg bg-muted">
    {#if getFavicon(entry.url)}
      <img src={getFavicon(entry.url)} alt="{getDomain(entry.url)} favicon" class="size-6" />
    {:else}
      <GlobeSimple class="size-5 text-muted-foreground" />
    {/if}
  </section>

  <section class="min-w-0 flex-1">
    <span class="truncate text-sm font-medium text-foreground">
      {getDomain(entry.url)}
    </span>
    <span class="truncate text-xs text-foreground-alt">
      {entry.username}
    </span>
  </section>

  <section class="hidden items-center text-muted-foreground sm:flex">
    <span class="text-xs tracking-widest">••••••••••</span>
  </section>

  <section class="flex items-center gap-x-1">
    {#each actions as { icon, title, action }, i (title + i)}
      <CardAction danger={title === 'Delete'} {icon} {title} {action} />
    {/each}
  </section>
</article>
