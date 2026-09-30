<script lang="ts">
  import Copy from 'phosphor-svelte/lib/CopyIcon'
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import CardAction from '@/components/card-action.svelte'
  import { CATEGORY_DEFAULT_COLOR } from '$lib/consts'
  import type { CardPasswordProps } from '@/components/types'

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

  let { entry, onedit, ondelete, oncopy, categories }: CardPasswordProps = $props()

  const color = $derived(
    categories.find((category) => category.id === entry.categoryId)?.color ?? CATEGORY_DEFAULT_COLOR,
  )
  const avatarStyle = $derived(avatarStyleFor(color))

  const initial = $derived(getInitial(entry.url))

  function getDomain(url: string): string {
    try {
      return new URL(url).hostname
    } catch {
      return url
    }
  }

  function getInitial(url: string): string {
    const domain = getDomain(url)
    const letter = domain.replace(/^(https?:\/\/)?www\./, '').charAt(0)
    return (letter || '?').toUpperCase()
  }

  function avatarStyleFor(hex: string): string {
    const value = hex.replace('#', '')
    const full =
      value.length === 3
        ? value
            .split('')
            .map((c) => c + c)
            .join('')
        : value
    const r = parseInt(full.slice(0, 2), 16)
    const g = parseInt(full.slice(2, 4), 16)
    const b = parseInt(full.slice(4, 6), 16)
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
    if (luminance < 0.25) return `background-color: ${hex}40; color: #f4f4f5`
    if (luminance > 0.85) return `background-color: ${hex}; color: #0a0a0a`
    return `background-color: ${hex}40; color: ${hex}`
  }

  async function copyPassword() {
    await oncopy(entry.id)
  }
</script>

<article
  class="flex items-center gap-x-4 rounded-card border-border-card bg-background-alt p-4 transition-colors hover:bg-muted"
>
  <section
    class="text-md flex size-10 items-center justify-center rounded-lg font-mono leading-none font-semibold uppercase select-none"
    style={avatarStyle}
  >
    {initial}
  </section>

  <section class="grid min-w-0 flex-1">
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
