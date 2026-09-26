<script lang="ts">
  import { AlertDialog } from 'bits-ui'
  import type { AlertProps } from '@/components/types'

  let { open = $bindable(false), title, description, actions }: AlertProps = $props()
</script>

<AlertDialog.Root bind:open>
  <AlertDialog.Portal>
    <AlertDialog.Overlay
      class="fixed inset-0 z-50 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
    />
    <AlertDialog.Content
      class="fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-card-lg border bg-background p-7 shadow-popover outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 sm:max-w-lg md:w-full"
    >
      <section class="flex flex-col gap-y-4 pb-6">
        <AlertDialog.Title class="text-lg font-semibold tracking-tight">{title}</AlertDialog.Title>
        {#if description && description.length > 0}
          <AlertDialog.Description class="text-foreground-alt text-sm">
            {#each description as paragraph, i (paragraph + i)}
              <p>{paragraph}</p>
            {/each}
          </AlertDialog.Description>
        {/if}
      </section>
      <section class="flex w-full items-center justify-center gap-2">
        {#each actions as { label, variant = 'secondary', action }, i (label + i)}
          {#if action}
            <AlertDialog.Action class={['btn', variant]} onclick={action}>{label}</AlertDialog.Action>
          {:else}
            <AlertDialog.Cancel class={['btn', variant]}>{label}</AlertDialog.Cancel>
          {/if}
        {/each}
      </section>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
