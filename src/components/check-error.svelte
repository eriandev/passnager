<script lang="ts">
  import Button from '@/components/button.svelte'
  import Warning from 'phosphor-svelte/lib/WarningIcon'

  let { message, onretry }: { message: string; onretry: () => void } = $props()
</script>

<main class="grid h-screen w-full place-items-center px-4">
  <section class="flex w-full max-w-md flex-col items-center gap-4 text-center">
    <Warning class="size-8 text-foreground-alt" />
    <h1 class="text-lg font-semibold tracking-tight">Could not read your vault</h1>
    <p class="text-foreground-alt text-sm">
      Passnager could not open its database, so it cannot tell whether a vault exists. Nothing has been changed. This
      usually clears on its own — if it does not, check that the database file is readable and not held by another copy
      of the app.
    </p>
    <p class="text-muted-foreground font-mono text-xs break-all">{message}</p>
    <Button onclick={onretry}>Try again</Button>
  </section>
</main>
